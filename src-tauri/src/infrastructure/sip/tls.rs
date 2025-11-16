// TLS certificate validation for SIPS connections
// Provides hostname extraction from SIP URIs and proper TLS certificate validation

use crate::domain::errors::SipError;
use std::net::IpAddr;
use std::str::FromStr;

/// Extract hostname from a SIP URI string
///
/// Supports various SIP URI formats:
/// - `sip:user@example.com`
/// - `sips:server.com:5061`
/// - `sip:example.com`
/// - `sip:192.168.1.1` (IP address - returns error)
///
/// Returns the hostname portion of the URI, or an error if:
/// - The URI is invalid
/// - The URI contains an IP address (not a hostname)
/// - The hostname cannot be extracted
pub fn extract_hostname_from_uri(uri: &str) -> Result<String, SipError> {
    // Remove angle brackets if present (e.g., <sip:user@example.com>)
    let uri = uri.trim().trim_start_matches('<').trim_end_matches('>');

    // Remove the scheme (sip: or sips:)
    let uri_without_scheme = if let Some(stripped) = uri.strip_prefix("sips:") {
        stripped
    } else if let Some(stripped) = uri.strip_prefix("sip:") {
        stripped
    } else {
        return Err(SipError::InvalidMessage {
            reason: format!("Invalid SIP URI format: {}", uri),
        });
    };

    // Extract the hostname part
    // URI format: [user[:password]@]hostname[:port][;parameters]
    let hostname_part = if let Some(at_pos) = uri_without_scheme.find('@') {
        // Has user part: extract everything after @
        &uri_without_scheme[at_pos + 1..]
    } else {
        // No user part: use the whole thing
        uri_without_scheme
    };

    // Remove port if present (e.g., example.com:5061)
    let hostname = if let Some(colon_pos) = hostname_part.find(':') {
        &hostname_part[..colon_pos]
    } else {
        hostname_part
    };

    // Remove parameters if present (e.g., example.com;transport=tcp)
    let hostname = if let Some(semicolon_pos) = hostname.find(';') {
        &hostname[..semicolon_pos]
    } else {
        hostname
    };

    // Remove trailing slash if present
    let hostname = hostname.trim_end_matches('/');

    // Validate that it's not an IP address
    if is_ip_address(hostname) {
        return Err(SipError::InvalidMessage {
            reason: format!(
                "URI contains IP address '{}' instead of hostname. Cannot use IP for TLS certificate validation.",
                hostname
            ),
        });
    }

    // Validate hostname is not empty
    if hostname.is_empty() {
        return Err(SipError::InvalidMessage {
            reason: "Hostname cannot be empty".to_string(),
        });
    }

    Ok(hostname.to_string())
}

/// Check if a string is an IP address (IPv4 or IPv6)
fn is_ip_address(s: &str) -> bool {
    // Check for IPv4 (simple check: contains dots and all parts are digits)
    if s.contains('.') {
        let parts: Vec<&str> = s.split('.').collect();
        if parts.len() == 4 && parts.iter().all(|part| part.parse::<u8>().is_ok()) {
            return true;
        }
    }

    // Check for IPv6 using standard library for robust validation
    IpAddr::from_str(s).is_ok()
}

/// Create a TLS client configuration with proper root certificates
///
/// This creates a rustls ClientConfig with:
/// - Safe default cipher suites
/// - WebPKI root certificates for certificate validation
/// - No client authentication (server-only TLS)
pub fn create_tls_config() -> rustls::ClientConfig {
    let mut root_store = rustls::RootCertStore::empty();
    root_store.add_trust_anchors(webpki_roots::TLS_SERVER_ROOTS.iter().map(|ta| {
        rustls::OwnedTrustAnchor::from_subject_spki_name_constraints(
            ta.subject.as_ref().to_vec(),
            ta.subject_public_key_info.as_ref().to_vec(),
            ta.name_constraints.as_ref().map(|nc| nc.as_ref().to_vec()),
        )
    }));

    rustls::ClientConfig::builder()
        .with_safe_defaults()
        .with_root_certificates(root_store)
        .with_no_client_auth()
}

/// Extract hostname from credentials
///
/// If the server field in credentials is a hostname (not an IP address),
/// returns it. Otherwise, returns an error indicating that a hostname is required
/// for TLS certificate validation.
pub fn extract_hostname_from_credentials(server: &str) -> Result<String, SipError> {
    // Check if it's an IP address
    if is_ip_address(server) {
        return Err(SipError::InvalidMessage {
            reason: format!(
                "Server '{}' is an IP address. TLS certificate validation requires a hostname.",
                server
            ),
        });
    }

    // Validate hostname is not empty
    if server.is_empty() {
        return Err(SipError::InvalidMessage {
            reason: "Server hostname cannot be empty".to_string(),
        });
    }

    Ok(server.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_hostname_from_uri_simple() {
        let uri = "sip:example.com";
        let result = extract_hostname_from_uri(uri);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "example.com");
    }

    #[test]
    fn test_extract_hostname_from_uri_with_user() {
        let uri = "sip:user@example.com";
        let result = extract_hostname_from_uri(uri);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "example.com");
    }

    #[test]
    fn test_extract_hostname_from_uri_with_port() {
        let uri = "sip:example.com:5060";
        let result = extract_hostname_from_uri(uri);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "example.com");
    }

    #[test]
    fn test_extract_hostname_from_uri_with_user_and_port() {
        let uri = "sip:user@example.com:5060";
        let result = extract_hostname_from_uri(uri);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "example.com");
    }

    #[test]
    fn test_extract_hostname_from_sips_uri() {
        let uri = "sips:server.com:5061";
        let result = extract_hostname_from_uri(uri);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "server.com");
    }

    #[test]
    fn test_extract_hostname_from_uri_with_angle_brackets() {
        let uri = "<sip:user@example.com>";
        let result = extract_hostname_from_uri(uri);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "example.com");
    }

    #[test]
    fn test_extract_hostname_from_uri_with_parameters() {
        let uri = "sip:user@example.com;transport=tcp";
        let result = extract_hostname_from_uri(uri);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "example.com");
    }

    #[test]
    fn test_extract_hostname_from_uri_with_trailing_slash() {
        let uri = "sip:example.com/";
        let result = extract_hostname_from_uri(uri);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "example.com");
    }

    #[test]
    fn test_extract_hostname_from_uri_ipv4_rejected() {
        let uri = "sip:192.168.1.1";
        let result = extract_hostname_from_uri(uri);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("IP address"));
    }

    #[test]
    fn test_extract_hostname_from_uri_ipv4_with_port_rejected() {
        let uri = "sip:192.168.1.1:5060";
        let result = extract_hostname_from_uri(uri);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("IP address"));
    }

    #[test]
    fn test_extract_hostname_from_uri_invalid_format() {
        let uri = "http://example.com";
        let result = extract_hostname_from_uri(uri);
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("Invalid SIP URI format"));
    }

    #[test]
    fn test_extract_hostname_from_uri_empty() {
        let uri = "sip:";
        let result = extract_hostname_from_uri(uri);
        assert!(result.is_err());
    }

    #[test]
    fn test_is_ip_address_ipv4() {
        assert!(is_ip_address("192.168.1.1"));
        assert!(is_ip_address("127.0.0.1"));
        assert!(is_ip_address("10.0.0.1"));
        assert!(!is_ip_address("example.com"));
        assert!(!is_ip_address("192.168.1"));
    }

    #[test]
    fn test_is_ip_address_ipv6() {
        assert!(is_ip_address("2001:0db8:85a3:0000:0000:8a2e:0370:7334"));
        assert!(is_ip_address("::1"));
        assert!(is_ip_address("2001:db8::1"));
    }

    #[test]
    fn test_extract_hostname_from_credentials_hostname() {
        let server = "sip.example.com";
        let result = extract_hostname_from_credentials(server);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "sip.example.com");
    }

    #[test]
    fn test_extract_hostname_from_credentials_ip_rejected() {
        let server = "192.168.1.1";
        let result = extract_hostname_from_credentials(server);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("IP address"));
    }

    #[test]
    fn test_extract_hostname_from_credentials_empty() {
        let server = "";
        let result = extract_hostname_from_credentials(server);
        assert!(result.is_err());
    }

    #[test]
    fn test_create_tls_config() {
        let config = create_tls_config();
        // Just verify it creates a config without panicking
        // The actual validation happens during TLS handshake
        // We can't directly access root_store, but we can verify the config was created
        // by checking that it's not the default (which would panic on use)
        // This test just ensures the function doesn't panic
        drop(config);
    }
}
