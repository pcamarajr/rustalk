// SIP header extraction utilities
// Shared functions for extracting headers from SIP messages

use crate::domain::errors::SipError;
use rsip::SipMessage;

/// Extract Call-ID header from a SIP message
pub fn extract_call_id_header(message: &SipMessage) -> Result<String, SipError> {
    // Convert SipMessage to bytes
    let message_bytes: Vec<u8> = message.clone().into();
    let message_str = String::from_utf8_lossy(&message_bytes);

    // Look for Call-ID header (case-insensitive, can be "Call-ID:" or "i:")
    for line in message_str.lines() {
        let line_lower = line.to_lowercase();
        if line_lower.starts_with("call-id:") || line_lower.starts_with("i:") {
            // Extract value after colon using split_once to limit splits to first colon only
            // This handles edge cases where the Call-ID value itself might contain a colon
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
pub fn extract_from_header(message: &SipMessage) -> Result<String, SipError> {
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
pub fn extract_to_header(message: &SipMessage) -> Result<String, SipError> {
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
