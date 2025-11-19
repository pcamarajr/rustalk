// SIP INVITE message construction
// Provides high-level API for building RFC 3261-compliant INVITE messages

use crate::domain::errors::SipError;
use crate::infrastructure::sip::message_builder::SipMessageBuilder;
use crate::infrastructure::sip::parser::parse_message;
use crate::infrastructure::sip::sdp::{generate_sdp_offer, SdpOfferParams};
use std::net::SocketAddr;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

// Static counter for ensuring uniqueness
static CALL_ID_COUNTER: AtomicU64 = AtomicU64::new(0);

/// Generate a unique Call-ID for SIP messages
///
/// Format: `{hash}@{domain}`
/// Uses timestamp + counter for uniqueness (RFC 3261 requirement)
///
/// # Arguments
/// * `local_uri` - Local SIP URI (e.g., "sip:alice@example.com")
///
/// # Returns
/// A unique Call-ID string
fn generate_call_id(local_uri: &str) -> Result<String, SipError> {
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|e| SipError::InvalidMessage {
            reason: format!("System time error: {}", e),
        })?
        .as_nanos();

    // Increment counter for additional uniqueness
    let counter = CALL_ID_COUNTER.fetch_add(1, Ordering::Relaxed);

    // Extract domain from URI (e.g., "sip:alice@example.com" -> "example.com")
    let domain = local_uri
        .split('@')
        .nth(1)
        .unwrap_or("localhost")
        .split(':')
        .next()
        .unwrap_or("localhost");

    // Generate hash from timestamp, counter, and URI for uniqueness
    let hash_input = format!("{}{}{}", timestamp, counter, local_uri);
    let hash = md5::compute(hash_input.as_bytes());
    let call_id = format!("{}@{}", hex::encode(hash.as_slice()), domain);

    Ok(call_id)
}

// Static counter for From tag uniqueness
static FROM_TAG_COUNTER: AtomicU64 = AtomicU64::new(0);

/// Generate a From tag for SIP messages
///
/// Uses timestamp + counter for uniqueness within call context
///
/// # Returns
/// A unique From tag string
fn generate_from_tag() -> Result<String, SipError> {
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|e| SipError::InvalidMessage {
            reason: format!("System time error: {}", e),
        })?
        .as_nanos();

    // Increment counter for additional uniqueness
    let counter = FROM_TAG_COUNTER.fetch_add(1, Ordering::Relaxed);

    Ok(format!("{}{}", timestamp, counter))
}

// Static counter for branch uniqueness
static BRANCH_COUNTER: AtomicU64 = AtomicU64::new(0);

/// Generate a Via branch parameter
///
/// Must start with `z9hG4bK` (RFC 3261 requirement)
/// Includes unique identifier (timestamp + counter)
///
/// # Returns
/// A unique branch parameter string starting with z9hG4bK
fn generate_branch() -> Result<String, SipError> {
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|e| SipError::InvalidMessage {
            reason: format!("System time error: {}", e),
        })?
        .as_nanos();

    // Increment counter for additional uniqueness
    let counter = BRANCH_COUNTER.fetch_add(1, Ordering::Relaxed);

    // Generate hash for uniqueness
    let hash_input = format!("{}branch{}", timestamp, counter);
    let hash = md5::compute(hash_input.as_bytes());
    let branch_suffix = hex::encode(hash.as_slice());

    Ok(format!("z9hG4bK{}", &branch_suffix[..16]))
}

/// Build an INVITE request message
///
/// Constructs a complete RFC 3261-compliant INVITE message with all required headers.
/// The message is validated using `parse_message` before being returned.
///
/// # Arguments
/// * `remote_uri` - Target SIP URI (e.g., "sip:bob@example.com")
/// * `local_uri` - Local SIP URI (from credentials, e.g., "sip:alice@example.com")
/// * `local_address` - Local socket address for Via header
/// * `contact_uri` - Contact header URI (e.g., "sip:alice@192.168.1.100:5060")
/// * `sdp_body` - Optional SDP body (string) - will be handled in OUT-2.2, but accept as parameter
/// * `cseq` - CSeq sequence number (default: 1)
///
/// # Returns
/// Raw INVITE message bytes, or `SipError` if construction fails
///
/// # Example
/// ```
/// use rustalk_lib::infrastructure::sip::invite::build_invite_request;
/// use std::net::SocketAddr;
///
/// let remote_uri = "sip:bob@example.com";
/// let local_uri = "sip:alice@example.com";
/// let local_addr: SocketAddr = "192.168.1.100:5060".parse().unwrap();
/// let contact_uri = "sip:alice@192.168.1.100:5060";
///
/// let invite = build_invite_request(
///     remote_uri,
///     local_uri,
///     &local_addr,
///     contact_uri,
///     None,
///     1,
/// ).unwrap();
/// ```
pub fn build_invite_request(
    remote_uri: &str,
    local_uri: &str,
    local_address: &SocketAddr,
    contact_uri: &str,
    sdp_body: Option<&str>,
    cseq: u32,
) -> Result<Vec<u8>, SipError> {
    // Generate unique identifiers
    let call_id = generate_call_id(local_uri)?;
    let from_tag = generate_from_tag()?;
    let branch = generate_branch()?;

    // Build Via header
    let via_header = format!("SIP/2.0/UDP {};branch={}", local_address, branch);

    // Build To header (may not have tag initially)
    let to_header = format!("<{}>", remote_uri);

    // Build From header with tag
    let from_header = format!("<{}>;tag={}", local_uri, from_tag);

    // Build CSeq header
    let cseq_header = format!("{} INVITE", cseq);

    // Start building the message
    let mut builder = SipMessageBuilder::new()
        .method("INVITE")
        .uri(remote_uri)
        .header("Via", &via_header)
        .header("Max-Forwards", "70")
        .header("To", &to_header)
        .header("From", &from_header)
        .header("Call-ID", &call_id)
        .header("CSeq", &cseq_header)
        .header("Contact", &format!("<{}>", contact_uri))
        .header("User-Agent", "RUSTALK/1.0");

    // Add Content-Type and body if SDP is provided
    if let Some(sdp) = sdp_body {
        builder = builder.header("Content-Type", "application/sdp").body(sdp);
    }

    // Build the message
    let message_bytes = builder.build()?;

    // Validate the built message by parsing it
    parse_message(&message_bytes)?;

    Ok(message_bytes)
}

/// Build an INVITE request with auto-generated SDP offer
///
/// This is a convenience function that automatically generates an SDP offer
/// for outbound calls. It extracts the IP address from the local_address
/// and generates appropriate SDP parameters.
///
/// # Arguments
/// * `remote_uri` - Target SIP URI (e.g., "sip:bob@example.com")
/// * `local_uri` - Local SIP URI (from credentials, e.g., "sip:alice@example.com")
/// * `local_address` - Local socket address for Via header
/// * `contact_uri` - Contact header URI (e.g., "sip:alice@192.168.1.100:5060")
/// * `rtp_port` - RTP port for audio (must be even)
/// * `username` - Username for SDP origin (typically from local_uri)
/// * `cseq` - CSeq sequence number (default: 1)
///
/// # Returns
/// Raw INVITE message bytes with SDP body, or `SipError` if construction fails
///
/// # Example
/// ```
/// use rustalk_lib::infrastructure::sip::invite::build_invite_with_sdp;
/// use std::net::SocketAddr;
///
/// let remote_uri = "sip:bob@example.com";
/// let local_uri = "sip:alice@example.com";
/// let local_addr: SocketAddr = "192.168.1.100:5060".parse().unwrap();
/// let contact_uri = "sip:alice@192.168.1.100:5060";
///
/// let invite = build_invite_with_sdp(
///     remote_uri,
///     local_uri,
///     &local_addr,
///     contact_uri,
///     49172,
///     "alice",
///     1,
/// ).unwrap();
/// ```
pub fn build_invite_with_sdp(
    remote_uri: &str,
    local_uri: &str,
    local_address: &SocketAddr,
    contact_uri: &str,
    rtp_port: u16,
    username: &str,
    cseq: u32,
) -> Result<Vec<u8>, SipError> {
    // Extract IP address from local_address
    let local_ip = local_address.ip();

    // Generate session ID (timestamp-based for uniqueness)
    let session_id = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|e| SipError::InvalidMessage {
            reason: format!("System time error: {}", e),
        })?
        .as_secs();

    // Generate SDP offer
    let sdp_params = SdpOfferParams {
        local_ip,
        rtp_port,
        username: username.to_string(),
        session_id,
    };

    let sdp_body = generate_sdp_offer(&sdp_params)?;

    // Build INVITE with SDP
    build_invite_request(
        remote_uri,
        local_uri,
        local_address,
        contact_uri,
        Some(&sdp_body),
        cseq,
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::infrastructure::sip::parser::parse_message;
    use rsip::SipMessage;

    #[test]
    fn test_build_invite_without_sdp() {
        let remote_uri = "sip:bob@example.com";
        let local_uri = "sip:alice@example.com";
        let local_addr: SocketAddr = "192.168.1.100:5060".parse().unwrap();
        let contact_uri = "sip:alice@192.168.1.100:5060";

        let result = build_invite_request(remote_uri, local_uri, &local_addr, contact_uri, None, 1);

        assert!(result.is_ok(), "Should build INVITE without SDP");
        let bytes = result.unwrap();
        assert!(!bytes.is_empty(), "Message should not be empty");
    }

    #[test]
    fn test_build_invite_with_sdp_body() {
        let remote_uri = "sip:bob@example.com";
        let local_uri = "sip:alice@example.com";
        let local_addr: SocketAddr = "192.168.1.100:5060".parse().unwrap();
        let contact_uri = "sip:alice@192.168.1.100:5060";
        let sdp_body = "v=0\r\n\
            o=alice 2890844526 2890844526 IN IP4 192.168.1.100\r\n\
            s=-\r\n\
            c=IN IP4 192.168.1.100\r\n\
            t=0 0\r\n\
            m=audio 49172 RTP/AVP 0\r\n\
            a=rtpmap:0 PCMU/8000\r\n";

        let result = build_invite_request(
            remote_uri,
            local_uri,
            &local_addr,
            contact_uri,
            Some(sdp_body),
            1,
        );

        assert!(result.is_ok(), "Should build INVITE with SDP");
        let bytes = result.unwrap();
        assert!(!bytes.is_empty(), "Message should not be empty");

        // Verify SDP is in the message
        let message_str = String::from_utf8_lossy(&bytes);
        assert!(message_str.contains("Content-Type: application/sdp"));
        assert!(message_str.contains("v=0"));
    }

    #[test]
    fn test_invite_headers_present() {
        let remote_uri = "sip:bob@example.com";
        let local_uri = "sip:alice@example.com";
        let local_addr: SocketAddr = "192.168.1.100:5060".parse().unwrap();
        let contact_uri = "sip:alice@192.168.1.100:5060";

        let bytes =
            build_invite_request(remote_uri, local_uri, &local_addr, contact_uri, None, 1).unwrap();
        let message_str = String::from_utf8_lossy(&bytes);

        // Check all required headers are present
        assert!(
            message_str.contains("INVITE"),
            "Should contain INVITE method"
        );
        assert!(message_str.contains("Via:"), "Should contain Via header");
        assert!(
            message_str.contains("Max-Forwards:"),
            "Should contain Max-Forwards header"
        );
        assert!(message_str.contains("To:"), "Should contain To header");
        assert!(message_str.contains("From:"), "Should contain From header");
        assert!(
            message_str.contains("Call-ID:"),
            "Should contain Call-ID header"
        );
        assert!(message_str.contains("CSeq:"), "Should contain CSeq header");
        assert!(
            message_str.contains("Contact:"),
            "Should contain Contact header"
        );
        assert!(
            message_str.contains("User-Agent:"),
            "Should contain User-Agent header"
        );
        assert!(
            message_str.contains("Content-Length:"),
            "Should contain Content-Length header"
        );
    }

    #[test]
    fn test_call_id_generation() {
        let local_uri = "sip:alice@example.com";
        let call_id1 = generate_call_id(local_uri).unwrap();
        let call_id2 = generate_call_id(local_uri).unwrap();

        // Call-IDs should be unique
        assert_ne!(call_id1, call_id2, "Call-IDs should be unique");

        // Call-ID should contain @ symbol (format: hash@domain)
        assert!(call_id1.contains('@'), "Call-ID should contain @");
        assert!(call_id2.contains('@'), "Call-ID should contain @");

        // Call-ID should end with domain
        assert!(
            call_id1.ends_with("@example.com"),
            "Call-ID should end with domain"
        );
        assert!(
            call_id2.ends_with("@example.com"),
            "Call-ID should end with domain"
        );
    }

    #[test]
    fn test_from_tag_generation() {
        let tag1 = generate_from_tag().unwrap();
        let tag2 = generate_from_tag().unwrap();

        // Tags should be unique (timestamp-based, so they should differ)
        assert_ne!(tag1, tag2, "From tags should be unique");
        assert!(!tag1.is_empty(), "From tag should not be empty");
        assert!(!tag2.is_empty(), "From tag should not be empty");
    }

    #[test]
    fn test_via_branch_generation() {
        let branch1 = generate_branch().unwrap();
        let branch2 = generate_branch().unwrap();

        // Branch should start with z9hG4bK (RFC 3261 requirement)
        assert!(
            branch1.starts_with("z9hG4bK"),
            "Branch should start with z9hG4bK"
        );
        assert!(
            branch2.starts_with("z9hG4bK"),
            "Branch should start with z9hG4bK"
        );

        // Branches should be unique
        assert_ne!(branch1, branch2, "Branches should be unique");
    }

    #[test]
    fn test_invite_parseable() {
        let remote_uri = "sip:bob@example.com";
        let local_uri = "sip:alice@example.com";
        let local_addr: SocketAddr = "192.168.1.100:5060".parse().unwrap();
        let contact_uri = "sip:alice@192.168.1.100:5060";

        let bytes =
            build_invite_request(remote_uri, local_uri, &local_addr, contact_uri, None, 1).unwrap();

        // Verify it can be parsed back
        let parsed = parse_message(&bytes);
        assert!(parsed.is_ok(), "Built message should be parseable");

        // Verify it's a request
        let message = parsed.unwrap();
        match message {
            SipMessage::Request(_) => {}
            SipMessage::Response(_) => panic!("Should be a request, not a response"),
        }
    }

    #[test]
    fn test_invite_content_length() {
        let remote_uri = "sip:bob@example.com";
        let local_uri = "sip:alice@example.com";
        let local_addr: SocketAddr = "192.168.1.100:5060".parse().unwrap();
        let contact_uri = "sip:alice@192.168.1.100:5060";
        let sdp_body = "v=0\r\n\
            o=alice 2890844526 2890844526 IN IP4 192.168.1.100\r\n\
            s=-\r\n\
            c=IN IP4 192.168.1.100\r\n\
            t=0 0\r\n\
            m=audio 49172 RTP/AVP 0\r\n\
            a=rtpmap:0 PCMU/8000\r\n";

        let bytes = build_invite_request(
            remote_uri,
            local_uri,
            &local_addr,
            contact_uri,
            Some(sdp_body),
            1,
        )
        .unwrap();

        let message_str = String::from_utf8_lossy(&bytes);
        let body_len = sdp_body.len();

        // Extract Content-Length value
        for line in message_str.lines() {
            if line.to_lowercase().starts_with("content-length:") {
                let content_length_str = line
                    .split(':')
                    .nth(1)
                    .unwrap_or("0")
                    .trim()
                    .parse::<usize>()
                    .unwrap_or(0);
                assert_eq!(
                    content_length_str, body_len,
                    "Content-Length should match body size"
                );
                return;
            }
        }

        panic!("Content-Length header not found");
    }

    #[test]
    fn test_invite_cseq_increment() {
        let remote_uri = "sip:bob@example.com";
        let local_uri = "sip:alice@example.com";
        let local_addr: SocketAddr = "192.168.1.100:5060".parse().unwrap();
        let contact_uri = "sip:alice@192.168.1.100:5060";

        let bytes1 =
            build_invite_request(remote_uri, local_uri, &local_addr, contact_uri, None, 1).unwrap();
        let bytes2 =
            build_invite_request(remote_uri, local_uri, &local_addr, contact_uri, None, 2).unwrap();

        let message_str1 = String::from_utf8_lossy(&bytes1);
        let message_str2 = String::from_utf8_lossy(&bytes2);

        // Check CSeq values
        assert!(
            message_str1.contains("CSeq: 1 INVITE"),
            "First message should have CSeq 1"
        );
        assert!(
            message_str2.contains("CSeq: 2 INVITE"),
            "Second message should have CSeq 2"
        );
    }

    #[test]
    fn test_invite_to_header_format() {
        let remote_uri = "sip:bob@example.com";
        let local_uri = "sip:alice@example.com";
        let local_addr: SocketAddr = "192.168.1.100:5060".parse().unwrap();
        let contact_uri = "sip:alice@192.168.1.100:5060";

        let bytes =
            build_invite_request(remote_uri, local_uri, &local_addr, contact_uri, None, 1).unwrap();
        let message_str = String::from_utf8_lossy(&bytes);

        // To header should be in format <sip:uri>
        assert!(
            message_str.contains("To: <sip:bob@example.com>"),
            "To header should be properly formatted"
        );
    }

    #[test]
    fn test_invite_from_header_with_tag() {
        let remote_uri = "sip:bob@example.com";
        let local_uri = "sip:alice@example.com";
        let local_addr: SocketAddr = "192.168.1.100:5060".parse().unwrap();
        let contact_uri = "sip:alice@192.168.1.100:5060";

        let bytes =
            build_invite_request(remote_uri, local_uri, &local_addr, contact_uri, None, 1).unwrap();
        let message_str = String::from_utf8_lossy(&bytes);

        // From header should contain tag
        assert!(
            message_str.contains("From: <sip:alice@example.com>;tag="),
            "From header should contain tag"
        );
    }

    #[test]
    fn test_invite_via_branch_format() {
        let remote_uri = "sip:bob@example.com";
        let local_uri = "sip:alice@example.com";
        let local_addr: SocketAddr = "192.168.1.100:5060".parse().unwrap();
        let contact_uri = "sip:alice@192.168.1.100:5060";

        let bytes =
            build_invite_request(remote_uri, local_uri, &local_addr, contact_uri, None, 1).unwrap();
        let message_str = String::from_utf8_lossy(&bytes);

        // Via header should contain branch starting with z9hG4bK
        let via_line = message_str
            .lines()
            .find(|line| line.to_lowercase().starts_with("via:"))
            .unwrap();
        assert!(
            via_line.contains("branch=z9hG4bK"),
            "Via header should contain branch starting with z9hG4bK"
        );
    }

    #[test]
    fn test_build_invite_with_sdp() {
        let remote_uri = "sip:bob@example.com";
        let local_uri = "sip:alice@example.com";
        let local_addr: SocketAddr = "192.168.1.100:5060".parse().unwrap();
        let contact_uri = "sip:alice@192.168.1.100:5060";

        let result = build_invite_with_sdp(
            remote_uri,
            local_uri,
            &local_addr,
            contact_uri,
            49172,
            "alice",
            1,
        );

        assert!(
            result.is_ok(),
            "Should build INVITE with auto-generated SDP"
        );
        let bytes = result.unwrap();
        assert!(!bytes.is_empty(), "Message should not be empty");

        // Verify SDP is in the message
        let message_str = String::from_utf8_lossy(&bytes);
        assert!(message_str.contains("Content-Type: application/sdp"));
        assert!(message_str.contains("v=0"));
        assert!(message_str.contains("m=audio 49172 RTP/AVP"));
    }

    #[test]
    fn test_build_invite_with_sdp_odd_port() {
        let remote_uri = "sip:bob@example.com";
        let local_uri = "sip:alice@example.com";
        let local_addr: SocketAddr = "192.168.1.100:5060".parse().unwrap();
        let contact_uri = "sip:alice@192.168.1.100:5060";

        let result = build_invite_with_sdp(
            remote_uri,
            local_uri,
            &local_addr,
            contact_uri,
            49173, // Odd port
            "alice",
            1,
        );

        assert!(result.is_err(), "Should reject odd RTP port");
    }
}
