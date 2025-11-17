// SIP registration with digest authentication (401 challenge/response flow)
// Implements RFC 2617 digest authentication for SIP REGISTER messages

use crate::domain::entities::credentials::Credentials;
use crate::domain::errors::SipError;
use crate::infrastructure::sip::client::SipClient;
use crate::infrastructure::sip::message_builder::SipMessageBuilder;
use crate::infrastructure::sip::parser::parse_message;
use rsip::SipMessage;
use std::net::SocketAddr;
use std::time::{SystemTime, UNIX_EPOCH};

/// Digest challenge parameters extracted from WWW-Authenticate header
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DigestChallenge {
    /// Authentication realm
    pub realm: String,
    /// Server-generated nonce
    pub nonce: String,
    /// Optional opaque value
    pub opaque: Option<String>,
    /// Digest algorithm (defaults to "MD5")
    pub algorithm: Option<String>,
    /// Quality of protection (e.g., "auth")
    pub qop: Option<String>,
}

/// Result of a registration attempt
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RegistrationResult {
    /// HTTP status code from the response
    pub status_code: u16,
    /// Expiration time in seconds (from Contact header expires parameter)
    pub expires: Option<u32>,
    /// Response message
    pub message: String,
}

/// Parse WWW-Authenticate or Proxy-Authenticate header from a 401/407 response
///
/// Extracts digest authentication challenge parameters from the header value.
/// Supports both quoted and unquoted parameter values.
///
/// # Arguments
/// * `header_value` - The value of the WWW-Authenticate or Proxy-Authenticate header
///
/// # Returns
/// A `DigestChallenge` struct with extracted parameters, or `SipError` if parsing fails
///
/// # Example
/// ```
/// use rustalk_lib::infrastructure::sip::registration::parse_www_authenticate;
///
/// let header = r#"Digest realm="example.com", nonce="dcd98b7102dd2f0e8b11d0f600bfb0c093", qop="auth""#;
/// let challenge = parse_www_authenticate(header).unwrap();
/// assert_eq!(challenge.realm, "example.com");
/// ```
pub fn parse_www_authenticate(header_value: &str) -> Result<DigestChallenge, SipError> {
    // Check if it's a Digest challenge
    if !header_value.trim_start().starts_with("Digest") {
        return Err(SipError::InvalidMessage {
            reason: "Not a Digest authentication challenge".to_string(),
        });
    }

    // Extract the part after "Digest"
    let digest_part = header_value
        .trim_start()
        .strip_prefix("Digest")
        .ok_or_else(|| SipError::InvalidMessage {
            reason: "Invalid WWW-Authenticate header format".to_string(),
        })?
        .trim_start();

    let mut realm = None;
    let mut nonce = None;
    let mut opaque = None;
    let mut algorithm = None;
    let mut qop = None;

    // Parse comma-separated key=value pairs
    // Handle both quoted and unquoted values
    let chars = digest_part.chars();
    let mut current_key = String::new();
    let mut current_value = String::new();
    let mut in_quotes = false;
    let mut in_key = true;

    for ch in chars {
        match ch {
            '=' if in_key && !in_quotes => {
                in_key = false;
            }
            '"' if !in_key => {
                in_quotes = !in_quotes;
            }
            ',' | ' ' if !in_quotes && !in_key => {
                // End of current parameter
                if !current_key.is_empty() && !current_value.is_empty() {
                    let key = current_key.trim().to_lowercase();
                    let value = current_value.trim().to_string();
                    match key.as_str() {
                        "realm" => realm = Some(value),
                        "nonce" => nonce = Some(value),
                        "opaque" => opaque = Some(value),
                        "algorithm" => algorithm = Some(value),
                        "qop" => qop = Some(value),
                        _ => {} // Ignore unknown parameters
                    }
                }
                current_key.clear();
                current_value.clear();
                in_key = true;
            }
            _ if in_key => {
                current_key.push(ch);
            }
            _ => {
                current_value.push(ch);
            }
        }
    }

    // Handle last parameter
    if !current_key.is_empty() && !current_value.is_empty() {
        let key = current_key.trim().to_lowercase();
        let value = current_value.trim().to_string();
        match key.as_str() {
            "realm" => realm = Some(value),
            "nonce" => nonce = Some(value),
            "opaque" => opaque = Some(value),
            "algorithm" => algorithm = Some(value),
            "qop" => qop = Some(value),
            _ => {}
        }
    }

    Ok(DigestChallenge {
        realm: realm.ok_or_else(|| SipError::InvalidMessage {
            reason: "Missing 'realm' in WWW-Authenticate header".to_string(),
        })?,
        nonce: nonce.ok_or_else(|| SipError::InvalidMessage {
            reason: "Missing 'nonce' in WWW-Authenticate header".to_string(),
        })?,
        opaque,
        algorithm,
        qop,
    })
}

/// Generate MD5 hash of a string
fn md5_hash(input: &str) -> String {
    let digest = md5::compute(input.as_bytes());
    hex::encode(digest.as_slice())
}

/// Generate Authorization header for REGISTER request with digest authentication
///
/// Implements RFC 2617 digest authentication:
/// - HA1 = MD5(username:realm:password)
/// - HA2 = MD5(method:uri)
/// - response = MD5(HA1:nonce:HA2) [or MD5(HA1:nonce:nc:cnonce:qop:HA2) if qop=auth]
///
/// # Arguments
/// * `method` - SIP method (e.g., "REGISTER")
/// * `uri` - Request URI
/// * `username` - SIP username
/// * `password` - SIP password
/// * `challenge` - Digest challenge from WWW-Authenticate header
///
/// # Returns
/// Authorization header value as a string, or `SipError` if generation fails
pub fn generate_authorization(
    method: &str,
    uri: &str,
    username: &str,
    password: &str,
    challenge: &DigestChallenge,
) -> Result<String, SipError> {
    // Calculate HA1 = MD5(username:realm:password)
    let ha1_input = format!("{}:{}:{}", username, challenge.realm, password);
    let ha1 = md5_hash(&ha1_input);

    // Calculate HA2 = MD5(method:uri)
    let ha2_input = format!("{}:{}", method, uri);
    let ha2 = md5_hash(&ha2_input);

    // Generate cnonce and nc if qop is present
    let (cnonce, nc) = if challenge.qop.is_some() {
        // Generate a random cnonce (client nonce)
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let cnonce_value = md5_hash(&format!("{}{}", timestamp, username));
        let nc_value = "00000001"; // Nonce count (incremented for each request)
        (Some(cnonce_value), Some(nc_value.to_string()))
    } else {
        (None, None)
    };

    // Calculate response
    let response = if let (Some(ref qop_val), Some(ref cnonce_val), Some(ref nc_val)) =
        (challenge.qop.as_ref(), cnonce.as_ref(), nc.as_ref())
    {
        // With qop: response = MD5(HA1:nonce:nc:cnonce:qop:HA2)
        let response_input = format!(
            "{}:{}:{}:{}:{}:{}",
            ha1, challenge.nonce, nc_val, cnonce_val, qop_val, ha2
        );
        md5_hash(&response_input)
    } else {
        // Without qop: response = MD5(HA1:nonce:HA2)
        let response_input = format!("{}:{}:{}", ha1, challenge.nonce, ha2);
        md5_hash(&response_input)
    };

    // Build Authorization header
    let mut auth_header = format!(
        r#"Digest username="{}", realm="{}", nonce="{}", uri="{}", response="{}""#,
        username, challenge.realm, challenge.nonce, uri, response
    );

    if let Some(ref opaque_val) = challenge.opaque {
        auth_header.push_str(&format!(r#", opaque="{}""#, opaque_val));
    }

    if let Some(ref algorithm_val) = challenge.algorithm {
        auth_header.push_str(&format!(r#", algorithm={}"#, algorithm_val));
    }

    if let (Some(ref qop_val), Some(ref cnonce_val), Some(ref nc_val)) =
        (challenge.qop.as_ref(), cnonce.as_ref(), nc.as_ref())
    {
        auth_header.push_str(&format!(
            r#", qop={}, cnonce="{}", nc={}"#,
            qop_val, cnonce_val, nc_val
        ));
    }

    Ok(auth_header)
}

/// Extract WWW-Authenticate header from a SIP response message
fn extract_www_authenticate(message: &SipMessage) -> Result<String, SipError> {
    // Convert message to bytes to extract headers
    let message_bytes: Vec<u8> = message.clone().into();
    let message_str = String::from_utf8_lossy(&message_bytes);

    // Look for WWW-Authenticate or Proxy-Authenticate header
    for line in message_str.lines() {
        let line_lower = line.to_lowercase();
        if line_lower.starts_with("www-authenticate:") {
            return Ok(line
                .strip_prefix("WWW-Authenticate:")
                .or_else(|| line.strip_prefix("www-authenticate:"))
                .unwrap_or("")
                .trim()
                .to_string());
        }
        if line_lower.starts_with("proxy-authenticate:") {
            return Ok(line
                .strip_prefix("Proxy-Authenticate:")
                .or_else(|| line.strip_prefix("proxy-authenticate:"))
                .unwrap_or("")
                .trim()
                .to_string());
        }
    }

    Err(SipError::MissingHeader {
        header: "WWW-Authenticate".to_string(),
    })
}

/// Extract status code from a SIP response message
fn extract_status_code(message: &SipMessage) -> Result<u16, SipError> {
    match message {
        SipMessage::Response(response) => {
            let status_str = response.status_code.to_string();
            status_str
                .parse::<u16>()
                .map_err(|_| SipError::InvalidMessage {
                    reason: format!("Invalid status code: {}", status_str),
                })
        }
        SipMessage::Request(_) => Err(SipError::InvalidMessage {
            reason: "Expected response message".to_string(),
        }),
    }
}

/// Extract expires value from Contact header (if present)
fn extract_expires(message: &SipMessage) -> Option<u32> {
    // Convert message to bytes to extract headers
    let message_bytes: Vec<u8> = message.clone().into();
    let message_str = String::from_utf8_lossy(&message_bytes);

    for line in message_str.lines() {
        let line_lower = line.to_lowercase();
        if line_lower.starts_with("contact:") {
            // Look for expires parameter: expires=3600
            if let Some(expires_part) = line.split("expires=").nth(1) {
                if let Some(expires_val) = expires_part.split(',').next() {
                    if let Ok(expires) = expires_val.trim().parse::<u32>() {
                        return Some(expires);
                    }
                }
            }
        }
    }

    None
}

/// Complete registration flow with 401 challenge/response
///
/// This function orchestrates the complete SIP registration flow:
/// 1. Send initial REGISTER request (without Authorization header)
/// 2. Receive and parse 401 Unauthorized response
/// 3. Extract challenge parameters from WWW-Authenticate header
/// 4. Generate Authorization header with digest
/// 5. Send second REGISTER request with Authorization header
/// 6. Return the final response result
///
/// # Arguments
/// * `client` - Mutable reference to SipClient for sending/receiving messages
/// * `credentials` - SIP account credentials
/// * `server_addr` - Server socket address
/// * `contact_uri` - Contact URI for registration (e.g., "sip:user@192.168.1.100:5060")
/// * `expires` - Registration expiration time in seconds (default: 3600)
///
/// # Returns
/// `RegistrationResult` with status code, expires, and message, or `SipError` if registration fails
pub async fn register_with_challenge(
    client: &mut SipClient,
    credentials: &Credentials,
    server_addr: &SocketAddr,
    contact_uri: &str,
    expires: u32,
) -> Result<RegistrationResult, SipError> {
    // Build server URI
    let server_uri = format!("sip:{}", credentials.server);

    // Generate Call-ID, CSeq, and tags
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
    let call_id = format!(
        "{}@{}",
        md5_hash(&format!("{}{}", timestamp, credentials.username)),
        credentials.server
    );
    let cseq = 1u32;
    let from_tag = format!("{}", timestamp);

    // Build To and From headers
    let to_uri = format!("sip:{}@{}", credentials.username, credentials.server);
    let from_uri = format!("sip:{}@{}", credentials.username, credentials.server);

    // Step 1: Send initial REGISTER request (without Authorization)
    let register_request = SipMessageBuilder::new()
        .method("REGISTER")
        .uri(&server_uri)
        .header(
            "Via",
            &format!(
                "SIP/2.0/UDP {};branch=z9hG4bK{}",
                client.local_address(),
                from_tag
            ),
        )
        .header("Max-Forwards", "70")
        .header("To", &format!("<{}>", to_uri))
        .header("From", &format!("<{}>;tag={}", from_uri, from_tag))
        .header("Call-ID", &call_id)
        .header("CSeq", &format!("{} REGISTER", cseq))
        .header("Contact", &format!("<{}>;expires={}", contact_uri, expires))
        .header("User-Agent", "RUSTALK/1.0")
        .build()?;

    // Parse the built message bytes to get SipMessage
    let register_message: SipMessage = parse_message(&register_request)?;

    client.send_message(&register_message, server_addr).await?;

    // Step 2: Receive 401 response
    let (response_message, _) = client.receive_message().await?;

    let status_code = extract_status_code(&response_message)?;

    // If we got 200 OK on first try, return success
    if status_code == 200 {
        return Ok(RegistrationResult {
            status_code: 200,
            expires: extract_expires(&response_message),
            message: "OK".to_string(),
        });
    }

    // Expect 401 Unauthorized
    if status_code != 401 {
        return Err(SipError::InvalidMessage {
            reason: format!("Expected 401 Unauthorized, got {}", status_code),
        });
    }

    // Step 3: Extract WWW-Authenticate header
    let www_auth_header = extract_www_authenticate(&response_message)?;
    let challenge = parse_www_authenticate(&www_auth_header)?;

    // Step 4: Generate Authorization header
    let auth_header = generate_authorization(
        "REGISTER",
        &server_uri,
        &credentials.username,
        &credentials.password,
        &challenge,
    )?;

    // Step 5: Send second REGISTER request with Authorization
    let register_request_auth = SipMessageBuilder::new()
        .method("REGISTER")
        .uri(&server_uri)
        .header(
            "Via",
            &format!(
                "SIP/2.0/UDP {};branch=z9hG4bK{}",
                client.local_address(),
                from_tag
            ),
        )
        .header("Max-Forwards", "70")
        .header("To", &format!("<{}>", to_uri))
        .header("From", &format!("<{}>;tag={}", from_uri, from_tag))
        .header("Call-ID", &call_id)
        .header("CSeq", &format!("{} REGISTER", cseq + 1))
        .header("Contact", &format!("<{}>;expires={}", contact_uri, expires))
        .header("Authorization", &auth_header)
        .header("User-Agent", "RUSTALK/1.0")
        .build()?;

    // Parse the built message bytes to get SipMessage
    let register_message_auth: SipMessage = parse_message(&register_request_auth)?;

    client
        .send_message(&register_message_auth, server_addr)
        .await?;

    // Step 6: Receive final response
    let (final_response, _) = client.receive_message().await?;
    let final_status_code = extract_status_code(&final_response)?;

    let status_message = match final_status_code {
        200 => "OK",
        403 => "Forbidden",
        408 => "Request Timeout",
        _ => "Unknown",
    };

    Ok(RegistrationResult {
        status_code: final_status_code,
        expires: extract_expires(&final_response),
        message: status_message.to_string(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_www_authenticate_basic() {
        let header = r#"Digest realm="example.com", nonce="dcd98b7102dd2f0e8b11d0f600bfb0c093""#;
        let challenge = parse_www_authenticate(header).unwrap();

        assert_eq!(challenge.realm, "example.com");
        assert_eq!(challenge.nonce, "dcd98b7102dd2f0e8b11d0f600bfb0c093");
        assert_eq!(challenge.opaque, None);
        assert_eq!(challenge.algorithm, None);
        assert_eq!(challenge.qop, None);
    }

    #[test]
    fn test_parse_www_authenticate_with_qop() {
        let header =
            r#"Digest realm="example.com", nonce="dcd98b7102dd2f0e8b11d0f600bfb0c093", qop="auth""#;
        let challenge = parse_www_authenticate(header).unwrap();

        assert_eq!(challenge.realm, "example.com");
        assert_eq!(challenge.nonce, "dcd98b7102dd2f0e8b11d0f600bfb0c093");
        assert_eq!(challenge.qop, Some("auth".to_string()));
    }

    #[test]
    fn test_parse_www_authenticate_with_all_params() {
        let header = r#"Digest realm="example.com", nonce="dcd98b7102dd2f0e8b11d0f600bfb0c093", opaque="5ccc069c403ebaf9f0171e9517f40e41", algorithm=MD5, qop="auth""#;
        let challenge = parse_www_authenticate(header).unwrap();

        assert_eq!(challenge.realm, "example.com");
        assert_eq!(challenge.nonce, "dcd98b7102dd2f0e8b11d0f600bfb0c093");
        assert_eq!(
            challenge.opaque,
            Some("5ccc069c403ebaf9f0171e9517f40e41".to_string())
        );
        assert_eq!(challenge.algorithm, Some("MD5".to_string()));
        assert_eq!(challenge.qop, Some("auth".to_string()));
    }

    #[test]
    fn test_parse_www_authenticate_missing_realm() {
        let header = r#"Digest nonce="dcd98b7102dd2f0e8b11d0f600bfb0c093""#;
        let result = parse_www_authenticate(header);
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_www_authenticate_missing_nonce() {
        let header = r#"Digest realm="example.com""#;
        let result = parse_www_authenticate(header);
        assert!(result.is_err());
    }

    #[test]
    fn test_generate_authorization_basic() {
        let challenge = DigestChallenge {
            realm: "example.com".to_string(),
            nonce: "dcd98b7102dd2f0e8b11d0f600bfb0c093".to_string(),
            opaque: None,
            algorithm: None,
            qop: None,
        };

        let auth = generate_authorization(
            "REGISTER",
            "sip:example.com",
            "user",
            "password",
            &challenge,
        )
        .unwrap();

        assert!(auth.contains("Digest"));
        assert!(auth.contains("username=\"user\""));
        assert!(auth.contains("realm=\"example.com\""));
        assert!(auth.contains("nonce=\"dcd98b7102dd2f0e8b11d0f600bfb0c093\""));
        assert!(auth.contains("uri=\"sip:example.com\""));
        assert!(auth.contains("response="));
    }

    #[test]
    fn test_generate_authorization_with_qop() {
        let challenge = DigestChallenge {
            realm: "example.com".to_string(),
            nonce: "dcd98b7102dd2f0e8b11d0f600bfb0c093".to_string(),
            opaque: None,
            algorithm: Some("MD5".to_string()),
            qop: Some("auth".to_string()),
        };

        let auth = generate_authorization(
            "REGISTER",
            "sip:example.com",
            "user",
            "password",
            &challenge,
        )
        .unwrap();

        assert!(auth.contains("qop=auth"));
        assert!(auth.contains("cnonce="));
        assert!(auth.contains("nc=00000001"));
    }

    #[test]
    fn test_md5_hash() {
        // Test MD5 hash calculation
        let hash = md5_hash("test");
        assert_eq!(hash.len(), 32); // MD5 produces 32 hex characters
    }
}
