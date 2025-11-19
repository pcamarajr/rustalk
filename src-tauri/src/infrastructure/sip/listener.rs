// SIP INVITE listener loop
// Continuously receives SIP messages, filters for INVITE requests, and routes them to CallService

use crate::domain::errors::SipError;
use crate::infrastructure::sip::client::SipClient;
use crate::services::call_service::CallService;
use rsip::SipMessage;
use std::sync::Arc;
use tokio::sync::Mutex;

/// Extract Call-ID header from a SIP message
fn extract_call_id_header(message: &SipMessage) -> Result<String, SipError> {
    // Convert SipMessage to bytes
    let message_bytes: Vec<u8> = message.clone().into();
    let message_str = String::from_utf8_lossy(&message_bytes);

    // Look for Call-ID header (case-insensitive, can be "Call-ID:" or "i:")
    for line in message_str.lines() {
        let line_lower = line.to_lowercase();
        if line_lower.starts_with("call-id:") || line_lower.starts_with("i:") {
            // Extract value after colon using split_once to limit splits to first colon only
            if let Some((_, call_id_value)) = line.split_once(':') {
                let call_id = call_id_value.trim().to_string();
                if !call_id.is_empty() {
                    return Ok(call_id);
                }
            }
        }
    }

    Err(SipError::InvalidMessage {
        reason: "Call-ID header not found in message".to_string(),
    })
}

/// Extract From header from a SIP message
fn extract_from_header(message: &SipMessage) -> Result<String, SipError> {
    let message_bytes: Vec<u8> = message.clone().into();
    let message_str = String::from_utf8_lossy(&message_bytes);

    for line in message_str.lines() {
        let line_lower = line.to_lowercase();
        if line_lower.starts_with("from:") || line_lower.starts_with("f:") {
            if let Some((_, from_value)) = line.split_once(':') {
                let from = from_value.trim().to_string();
                if !from.is_empty() {
                    return Ok(from);
                }
            }
        }
    }

    Err(SipError::InvalidMessage {
        reason: "From header not found in message".to_string(),
    })
}

/// Extract To header from a SIP message
fn extract_to_header(message: &SipMessage) -> Result<String, SipError> {
    let message_bytes: Vec<u8> = message.clone().into();
    let message_str = String::from_utf8_lossy(&message_bytes);

    for line in message_str.lines() {
        let line_lower = line.to_lowercase();
        if line_lower.starts_with("to:") || line_lower.starts_with("t:") {
            if let Some((_, to_value)) = line.split_once(':') {
                let to = to_value.trim().to_string();
                if !to.is_empty() {
                    return Ok(to);
                }
            }
        }
    }

    Err(SipError::InvalidMessage {
        reason: "To header not found in message".to_string(),
    })
}

/// Extract From tag from From header
fn extract_from_tag(from_header: &str) -> Option<String> {
    if let Some(tag_part) = from_header.split("tag=").nth(1) {
        let tag = tag_part.split(';').next().map(|s| s.trim().to_string());
        if let Some(t) = tag {
            if !t.is_empty() {
                return Some(t);
            }
        }
    }
    None
}

/// Extract Request-URI from a SIP request message
fn extract_request_uri(message: &SipMessage) -> Result<String, SipError> {
    match message {
        SipMessage::Request(_request) => {
            // The request URI is in the request line
            let message_bytes: Vec<u8> = message.clone().into();
            let message_str = String::from_utf8_lossy(&message_bytes);

            // Request line format: METHOD URI SIP/2.0
            if let Some(first_line) = message_str.lines().next() {
                let parts: Vec<&str> = first_line.split_whitespace().collect();
                if parts.len() >= 2 {
                    return Ok(parts[1].to_string());
                }
            }

            Err(SipError::InvalidMessage {
                reason: "Request-URI not found in request line".to_string(),
            })
        }
        SipMessage::Response(_) => Err(SipError::InvalidMessage {
            reason: "Expected request message".to_string(),
        }),
    }
}

/// Extract Via header from a SIP message
fn extract_via_header(message: &SipMessage) -> Result<String, SipError> {
    let message_bytes: Vec<u8> = message.clone().into();
    let message_str = String::from_utf8_lossy(&message_bytes);

    for line in message_str.lines() {
        let line_lower = line.to_lowercase();
        if line_lower.starts_with("via:") || line_lower.starts_with("v:") {
            if let Some((_, via_value)) = line.split_once(':') {
                let via = via_value.trim().to_string();
                if !via.is_empty() {
                    return Ok(via);
                }
            }
        }
    }

    Err(SipError::InvalidMessage {
        reason: "Via header not found in message".to_string(),
    })
}

/// Extract CSeq header from a SIP message
fn extract_cseq_header(message: &SipMessage) -> Result<String, SipError> {
    let message_bytes: Vec<u8> = message.clone().into();
    let message_str = String::from_utf8_lossy(&message_bytes);

    for line in message_str.lines() {
        let line_lower = line.to_lowercase();
        if line_lower.starts_with("cseq:") || line_lower.starts_with("c:") {
            if let Some((_, cseq_value)) = line.split_once(':') {
                let cseq = cseq_value.trim().to_string();
                if !cseq.is_empty() {
                    return Ok(cseq);
                }
            }
        }
    }

    Err(SipError::InvalidMessage {
        reason: "CSeq header not found in message".to_string(),
    })
}

/// Extract SDP body from a SIP message if present
fn extract_sdp_body(message: &SipMessage) -> Option<String> {
    // Convert SipMessage to bytes
    let message_bytes: Vec<u8> = message.clone().into();
    let message_str = String::from_utf8_lossy(&message_bytes);

    // Check for Content-Type: application/sdp header
    let mut has_sdp_content_type = false;
    let mut content_length: Option<usize> = None;

    for line in message_str.lines() {
        let line_lower = line.to_lowercase();
        if line_lower.starts_with("content-type:") {
            if line_lower.contains("application/sdp") {
                has_sdp_content_type = true;
            }
        } else if line_lower.starts_with("content-length:") {
            if let Some((_, length_str)) = line.split_once(':') {
                if let Ok(length) = length_str.trim().parse::<usize>() {
                    content_length = Some(length);
                }
            }
        }
    }

    // If we have SDP content type, extract the body
    if has_sdp_content_type {
        // Find the body separator (\r\n\r\n)
        if let Some(body_start) = message_str.find("\r\n\r\n") {
            let body = message_str[body_start + 4..].to_string();
            // If Content-Length is specified, use it to trim the body
            if let Some(length) = content_length {
                if body.len() >= length {
                    if body.len() != length {
                        eprintln!(
                            "DEBUG:[INVITE_LISTENER/SDP] Body length ({}) doesn't match Content-Length ({}), using Content-Length",
                            body.len(),
                            length
                        );
                    }
                    return Some(body[..length].to_string());
                } else {
                    eprintln!(
                        "DEBUG:[INVITE_LISTENER/SDP] Body length ({}) is less than Content-Length ({}), message may be incomplete",
                        body.len(),
                        length
                    );
                    return Some(body);
                }
            }
            // Otherwise, return the entire body after separator
            if !body.trim().is_empty() {
                return Some(body.trim().to_string());
            }
        }
    }

    None
}

/// Start the INVITE listener loop
///
/// This function runs in an infinite loop, continuously receiving SIP messages
/// from the SIP client, filtering for INVITE requests, and routing them to CallService
/// for handling incoming calls.
///
/// # Arguments
/// * `sip_client` - Arc<Mutex<SipClient>> for receiving messages
/// * `call_service` - Arc<Mutex<CallService>> for handling incoming INVITEs
pub async fn start_invite_listener(
    sip_client: Arc<Mutex<SipClient>>,
    call_service: Arc<Mutex<CallService>>,
) {
    eprintln!("DEBUG:[INVITE_LISTENER/START] Starting INVITE listener loop");

    loop {
        // Receive message from SIP client
        let result = {
            let mut client = sip_client.lock().await;
            client.receive_message().await
        };

        match result {
            Ok((message, source_addr)) => {
                eprintln!("DEBUG:[INVITE_LISTENER/RECEIVE] Received SIP message");

                // Filter for INVITE requests only (skip responses and other requests)
                let is_invite_request = match &message {
                    SipMessage::Request(request) => {
                        request.method.to_string().to_uppercase() == "INVITE"
                    }
                    SipMessage::Response(_) => false,
                };

                if !is_invite_request {
                    eprintln!("DEBUG:[INVITE_LISTENER/RECEIVE] Skipping non-INVITE message");
                    continue;
                }

                eprintln!("DEBUG:[INVITE_LISTENER/RECEIVE] Received INVITE request");

                // Extract Call-ID header
                let call_id_header = match extract_call_id_header(&message) {
                    Ok(call_id) => {
                        eprintln!(
                            "DEBUG:[INVITE_LISTENER/EXTRACT] Extracted Call-ID: {}",
                            call_id
                        );
                        call_id
                    }
                    Err(e) => {
                        eprintln!(
                            "DEBUG:[INVITE_LISTENER/EXTRACT] Failed to extract Call-ID: {}",
                            e
                        );
                        continue; // Skip this message
                    }
                };

                // Extract From header
                let from_header = match extract_from_header(&message) {
                    Ok(from) => {
                        eprintln!(
                            "DEBUG:[INVITE_LISTENER/EXTRACT] Extracted From: {}",
                            from
                        );
                        from
                    }
                    Err(e) => {
                        eprintln!(
                            "DEBUG:[INVITE_LISTENER/EXTRACT] Failed to extract From header: {}",
                            e
                        );
                        continue; // Skip this message
                    }
                };

                // Extract From tag
                let from_tag = extract_from_tag(&from_header);
                if from_tag.is_some() {
                    eprintln!(
                        "DEBUG:[INVITE_LISTENER/EXTRACT] Extracted From tag: {}",
                        from_tag.as_ref().unwrap()
                    );
                } else {
                    eprintln!("DEBUG:[INVITE_LISTENER/EXTRACT] Warning: From tag not found");
                }

                // Extract To header
                let to_header = match extract_to_header(&message) {
                    Ok(to) => {
                        eprintln!("DEBUG:[INVITE_LISTENER/EXTRACT] Extracted To: {}", to);
                        to
                    }
                    Err(e) => {
                        eprintln!(
                            "DEBUG:[INVITE_LISTENER/EXTRACT] Failed to extract To header: {}",
                            e
                        );
                        continue; // Skip this message
                    }
                };

                // Extract Request-URI (remote number)
                let remote_uri = match extract_request_uri(&message) {
                    Ok(uri) => {
                        eprintln!(
                            "DEBUG:[INVITE_LISTENER/EXTRACT] Extracted Request-URI: {}",
                            uri
                        );
                        uri
                    }
                    Err(e) => {
                        eprintln!(
                            "DEBUG:[INVITE_LISTENER/EXTRACT] Failed to extract Request-URI: {}",
                            e
                        );
                        continue; // Skip this message
                    }
                };

                // Extract SDP body (if present)
                let sdp_body = extract_sdp_body(&message);
                if sdp_body.is_some() {
                    eprintln!("DEBUG:[INVITE_LISTENER/EXTRACT] Extracted SDP body");
                }

                // Extract Via header (needed for 100 Trying response)
                let via_header = match extract_via_header(&message) {
                    Ok(via) => {
                        eprintln!("DEBUG:[INVITE_LISTENER/EXTRACT] Extracted Via: {}", via);
                        via
                    }
                    Err(e) => {
                        eprintln!(
                            "DEBUG:[INVITE_LISTENER/EXTRACT] Failed to extract Via header: {}",
                            e
                        );
                        continue; // Skip this message
                    }
                };

                // Extract CSeq header (needed for 100 Trying response)
                let cseq_header = match extract_cseq_header(&message) {
                    Ok(cseq) => {
                        eprintln!("DEBUG:[INVITE_LISTENER/EXTRACT] Extracted CSeq: {}", cseq);
                        cseq
                    }
                    Err(e) => {
                        eprintln!(
                            "DEBUG:[INVITE_LISTENER/EXTRACT] Failed to extract CSeq header: {}",
                            e
                        );
                        continue; // Skip this message
                    }
                };

                // Route to CallService.handle_incoming_invite()
                let result = {
                    let service = call_service.lock().await;
                    service
                        .handle_incoming_invite(
                            &call_id_header,
                            from_tag.as_deref(),
                            &from_header,
                            &to_header,
                            &remote_uri,
                            sdp_body.as_deref(),
                            &via_header,
                            &cseq_header,
                            source_addr,
                        )
                        .await
                };

                match result {
                    Ok(call_id) => {
                        eprintln!(
                            "DEBUG:[INVITE_LISTENER/HANDLE] Successfully handled incoming INVITE, created call: {}",
                            call_id.as_str()
                        );
                    }
                    Err(e) => {
                        eprintln!(
                            "DEBUG:[INVITE_LISTENER/HANDLE] Failed to handle incoming INVITE: {}",
                            e
                        );
                        // Continue loop despite error
                    }
                }
            }
            Err(e) => {
                eprintln!(
                    "DEBUG:[INVITE_LISTENER/RECEIVE] Error receiving message: {}",
                    e
                );
                // Continue loop despite error
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::infrastructure::sip::parser::parse_message;

    #[test]
    fn test_extract_call_id_from_invite() {
        let invite_msg = b"INVITE sip:bob@example.com SIP/2.0\r\n\
            Via: SIP/2.0/UDP client.example.com:5060;branch=z9hG4bK776asdhds\r\n\
            Max-Forwards: 70\r\n\
            To: <sip:bob@example.com>\r\n\
            From: <sip:alice@example.com>;tag=1928301774\r\n\
            Call-ID: a84b4c76e66710\r\n\
            CSeq: 1 INVITE\r\n\
            Contact: <sip:alice@client.example.com>\r\n\
            Content-Length: 0\r\n\r\n";

        let message = parse_message(invite_msg).unwrap();
        let call_id = extract_call_id_header(&message).unwrap();
        assert_eq!(call_id, "a84b4c76e66710");
    }

    #[test]
    fn test_extract_from_header() {
        let invite_msg = b"INVITE sip:bob@example.com SIP/2.0\r\n\
            From: <sip:alice@example.com>;tag=1928301774\r\n\
            Call-ID: a84b4c76e66710\r\n\
            Content-Length: 0\r\n\r\n";

        let message = parse_message(invite_msg).unwrap();
        let from = extract_from_header(&message).unwrap();
        assert!(from.contains("alice@example.com"));
        assert!(from.contains("tag=1928301774"));
    }

    #[test]
    fn test_extract_from_tag() {
        let from_header = "<sip:alice@example.com>;tag=1928301774";
        let tag = extract_from_tag(from_header).unwrap();
        assert_eq!(tag, "1928301774");
    }

    #[test]
    fn test_extract_to_header() {
        let invite_msg = b"INVITE sip:bob@example.com SIP/2.0\r\n\
            To: <sip:bob@example.com>\r\n\
            From: <sip:alice@example.com>;tag=1928301774\r\n\
            Call-ID: a84b4c76e66710\r\n\
            Content-Length: 0\r\n\r\n";

        let message = parse_message(invite_msg).unwrap();
        let to = extract_to_header(&message).unwrap();
        assert!(to.contains("bob@example.com"));
    }

    #[test]
    fn test_extract_request_uri() {
        let invite_msg = b"INVITE sip:bob@example.com SIP/2.0\r\n\
            From: <sip:alice@example.com>;tag=1928301774\r\n\
            Call-ID: a84b4c76e66710\r\n\
            Content-Length: 0\r\n\r\n";

        let message = parse_message(invite_msg).unwrap();
        let uri = extract_request_uri(&message).unwrap();
        assert_eq!(uri, "sip:bob@example.com");
    }

    #[test]
    fn test_extract_sdp_body() {
        let invite_msg = b"INVITE sip:bob@example.com SIP/2.0\r\n\
            Via: SIP/2.0/UDP client.example.com:5060;branch=z9hG4bK776asdhds\r\n\
            Max-Forwards: 70\r\n\
            To: <sip:bob@example.com>\r\n\
            From: <sip:alice@example.com>;tag=1928301774\r\n\
            Call-ID: a84b4c76e66710\r\n\
            CSeq: 1 INVITE\r\n\
            Contact: <sip:alice@client.example.com>\r\n\
            Content-Type: application/sdp\r\n\
            Content-Length: 142\r\n\r\n\
            v=0\r\n\
            o=alice 2890844526 2890844526 IN IP4 client.example.com\r\n\
            s=-\r\n\
            c=IN IP4 192.0.2.101\r\n\
            t=0 0\r\n\
            m=audio 49172 RTP/AVP 0\r\n\
            a=rtpmap:0 PCMU/8000\r\n";

        let message = parse_message(invite_msg).unwrap();
        let sdp = extract_sdp_body(&message);
        assert!(sdp.is_some());
        let sdp_str = sdp.unwrap();
        assert!(sdp_str.contains("v=0"));
        assert!(sdp_str.contains("m=audio"));
    }

    #[test]
    fn test_extract_call_id_missing() {
        let invite_msg = b"INVITE sip:bob@example.com SIP/2.0\r\n\
            From: <sip:alice@example.com>;tag=1928301774\r\n\
            Content-Length: 0\r\n\r\n";

        let message = parse_message(invite_msg).unwrap();
        let result = extract_call_id_header(&message);
        assert!(result.is_err());
    }
}

