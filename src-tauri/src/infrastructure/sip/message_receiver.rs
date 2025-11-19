// SIP message receiver loop
// Continuously receives SIP messages, parses responses, and routes them to CallService

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

/// Extract status code from a SIP response message
fn extract_status_code(message: &SipMessage) -> Result<u16, SipError> {
    match message {
        SipMessage::Response(response) => {
            let status_str = response.status_code.to_string();
            // The status_code might include the reason phrase (e.g., "401 Unauthorized")
            // Extract just the numeric part
            let numeric_part = status_str.split_whitespace().next().unwrap_or(&status_str);

            numeric_part
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
            // Extract value after colon using split_once to limit splits to first colon only
            // This handles edge cases where the value itself might contain a colon
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
                    // Validate that body length matches Content-Length exactly
                    if body.len() != length {
                        eprintln!(
                            "DEBUG:[MESSAGE_RECEIVER/SDP] Body length ({}) doesn't match Content-Length ({}), using Content-Length",
                            body.len(),
                            length
                        );
                    }
                    return Some(body[..length].to_string());
                } else {
                    // Handle incomplete message case
                    eprintln!(
                        "DEBUG:[MESSAGE_RECEIVER/SDP] Body length ({}) is less than Content-Length ({}), message may be incomplete",
                        body.len(),
                        length
                    );
                    // Return what we have, but log the issue
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

/// Start the message receiver loop
///
/// This function runs in an infinite loop, continuously receiving SIP messages
/// from the SIP client, parsing responses, and routing them to CallService
/// for proper call state transitions.
///
/// # Arguments
/// * `sip_client` - Arc<Mutex<SipClient>> for receiving messages
/// * `call_service` - Arc<Mutex<CallService>> for handling responses
pub async fn start_message_receiver(
    sip_client: Arc<Mutex<SipClient>>,
    call_service: Arc<Mutex<CallService>>,
) {
    eprintln!("DEBUG:[MESSAGE_RECEIVER/START] Starting message receiver loop");

    loop {
        // Receive message from SIP client
        let result = {
            let mut client = sip_client.lock().await;
            client.receive_message().await
        };

        match result {
            Ok((message, _source_addr)) => {
                eprintln!("DEBUG:[MESSAGE_RECEIVER/RECEIVE] Received SIP message");

                // Filter for response messages only (skip requests)
                let is_response = matches!(message, SipMessage::Response(_));
                if !is_response {
                    eprintln!("DEBUG:[MESSAGE_RECEIVER/RECEIVE] Skipping request message");
                    continue;
                }

                // Extract Call-ID header
                let call_id_header = match extract_call_id_header(&message) {
                    Ok(call_id) => {
                        eprintln!(
                            "DEBUG:[MESSAGE_RECEIVER/EXTRACT] Extracted Call-ID: {}",
                            call_id
                        );
                        call_id
                    }
                    Err(e) => {
                        eprintln!(
                            "DEBUG:[MESSAGE_RECEIVER/EXTRACT] Failed to extract Call-ID: {}",
                            e
                        );
                        continue; // Skip this message
                    }
                };

                // Extract status code
                let status_code = match extract_status_code(&message) {
                    Ok(code) => {
                        eprintln!(
                            "DEBUG:[MESSAGE_RECEIVER/EXTRACT] Extracted status code: {}",
                            code
                        );
                        code
                    }
                    Err(e) => {
                        eprintln!(
                            "DEBUG:[MESSAGE_RECEIVER/EXTRACT] Failed to extract status code: {}",
                            e
                        );
                        continue; // Skip this message
                    }
                };

                // Extract SDP body (if present)
                let sdp_body = extract_sdp_body(&message);
                if sdp_body.is_some() {
                    eprintln!("DEBUG:[MESSAGE_RECEIVER/EXTRACT] Extracted SDP body");
                }

                // Find call by Call-ID header
                let call_id_opt = {
                    let service = call_service.lock().await;
                    service.find_call_by_call_id_header(&call_id_header).await
                };

                match call_id_opt {
                    Some(call_id) => {
                        eprintln!(
                            "DEBUG:[MESSAGE_RECEIVER/MATCH] Matched Call-ID to call: {}",
                            call_id.as_str()
                        );

                        // Route to CallService.handle_invite_response()
                        let result = {
                            let service = call_service.lock().await;
                            service
                                .handle_invite_response(&call_id, status_code, sdp_body.as_deref())
                                .await
                        };

                        match result {
                            Ok(()) => {
                                eprintln!(
                                    "DEBUG:[MESSAGE_RECEIVER/HANDLE] Successfully handled response for call: {}",
                                    call_id.as_str()
                                );
                            }
                            Err(e) => {
                                eprintln!(
                                    "DEBUG:[MESSAGE_RECEIVER/HANDLE] Failed to handle response for call {}: {}",
                                    call_id.as_str(),
                                    e
                                );
                                // Continue loop despite error
                            }
                        }
                    }
                    None => {
                        eprintln!(
                            "DEBUG:[MESSAGE_RECEIVER/MATCH] No active call found for Call-ID: {}",
                            call_id_header
                        );
                        // Continue loop - this might be a response for a call that already ended
                    }
                }
            }
            Err(e) => {
                eprintln!(
                    "DEBUG:[MESSAGE_RECEIVER/RECEIVE] Error receiving message: {}",
                    e
                );
                // Continue loop despite error
            }
        }
    }
}
