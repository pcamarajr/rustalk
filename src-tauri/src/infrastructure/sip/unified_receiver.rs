// Unified SIP message receiver
// Single receiver loop that demultiplexes messages to appropriate handlers
// Fixes race condition where two separate loops competed for the same socket

use crate::domain::errors::SipError;
use crate::infrastructure::sip::client::SipClient;
use crate::infrastructure::sip::headers::{
    extract_call_id_header, extract_from_header, extract_to_header,
};
use crate::infrastructure::sip::listener::{
    extract_cseq_header, extract_from_tag, extract_request_uri,
    extract_sdp_body as extract_sdp_body_listener, extract_via_header,
};
use crate::infrastructure::sip::message_receiver::{
    extract_sdp_body as extract_sdp_body_receiver, extract_status_code,
};
use crate::services::call_service::CallService;
use rsip::SipMessage;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::sync::Mutex;

/// Start the unified message receiver loop
///
/// This function runs in an infinite loop, continuously receiving SIP messages
/// from the SIP client and demultiplexing them to appropriate handlers:
/// - INVITE requests → handle_incoming_invite()
/// - Response messages → handle_invite_response()
/// - Other messages → logged and skipped
///
/// This fixes the race condition where two separate loops competed for the same socket,
/// causing packet loss when messages were consumed by the wrong loop.
///
/// # Arguments
/// * `sip_client` - Arc<Mutex<SipClient>> for receiving messages
/// * `call_service` - Arc<Mutex<CallService>> for handling messages
pub async fn start_unified_receiver(
    sip_client: Arc<Mutex<SipClient>>,
    call_service: Arc<Mutex<CallService>>,
) {
    eprintln!("DEBUG:[UNIFIED_RECEIVER/START] Starting unified message receiver loop");

    loop {
        // Receive message from SIP client (single point of reception)
        let result = {
            let mut client = sip_client.lock().await;
            client.receive_message().await
        };

        match result {
            Ok((message, source_addr)) => {
                eprintln!("DEBUG:[UNIFIED_RECEIVER/RECEIVE] Received SIP message");

                // Demultiplex based on message type
                match &message {
                    // Handle INVITE requests
                    SipMessage::Request(request)
                        if request.method.to_string().to_uppercase() == "INVITE" =>
                    {
                        eprintln!(
                            "DEBUG:[UNIFIED_RECEIVER/DEMUX] Routing INVITE request to handler"
                        );
                        if let Err(e) =
                            handle_invite_request(&message, source_addr, &call_service).await
                        {
                            eprintln!(
                                "DEBUG:[UNIFIED_RECEIVER/HANDLE] Failed to handle INVITE request: {}",
                                e
                            );
                            // Continue loop despite error
                        }
                    }
                    // Handle response messages
                    SipMessage::Response(_) => {
                        eprintln!("DEBUG:[UNIFIED_RECEIVER/DEMUX] Routing response to handler");
                        if let Err(e) = handle_response(&message, &call_service).await {
                            eprintln!(
                                "DEBUG:[UNIFIED_RECEIVER/HANDLE] Failed to handle response: {}",
                                e
                            );
                            // Continue loop despite error
                        }
                    }
                    // Other requests (ACK, BYE, etc.) - log and skip for now
                    SipMessage::Request(req) => {
                        eprintln!(
                            "DEBUG:[UNIFIED_RECEIVER/DEMUX] Skipping non-INVITE request: {}",
                            req.method
                        );
                    }
                }
            }
            Err(e) => {
                eprintln!(
                    "DEBUG:[UNIFIED_RECEIVER/RECEIVE] Error receiving message: {}",
                    e
                );
                // Continue loop despite error
            }
        }
    }
}

/// Handle an incoming INVITE request
async fn handle_invite_request(
    message: &SipMessage,
    source_addr: SocketAddr,
    call_service: &Arc<Mutex<CallService>>,
) -> Result<(), SipError> {
    eprintln!("DEBUG:[UNIFIED_RECEIVER/INVITE] Processing INVITE request");

    // Extract Call-ID header
    let call_id_header = extract_call_id_header(message)?;
    eprintln!(
        "DEBUG:[UNIFIED_RECEIVER/INVITE] Extracted Call-ID: {}",
        call_id_header
    );

    // Extract From header
    let from_header = extract_from_header(message)?;
    eprintln!(
        "DEBUG:[UNIFIED_RECEIVER/INVITE] Extracted From: {}",
        from_header
    );

    // Extract From tag
    let from_tag = extract_from_tag(&from_header);
    if from_tag.is_some() {
        eprintln!(
            "DEBUG:[UNIFIED_RECEIVER/INVITE] Extracted From tag: {}",
            from_tag.as_ref().unwrap()
        );
    } else {
        eprintln!("DEBUG:[UNIFIED_RECEIVER/INVITE] Warning: From tag not found");
    }

    // Extract To header
    let to_header = extract_to_header(message)?;
    eprintln!(
        "DEBUG:[UNIFIED_RECEIVER/INVITE] Extracted To: {}",
        to_header
    );

    // Extract Request-URI (remote number)
    let remote_uri = extract_request_uri(message)?;
    eprintln!(
        "DEBUG:[UNIFIED_RECEIVER/INVITE] Extracted Request-URI: {}",
        remote_uri
    );

    // Extract SDP body (if present)
    let sdp_body = extract_sdp_body_listener(message);
    if sdp_body.is_some() {
        eprintln!("DEBUG:[UNIFIED_RECEIVER/INVITE] Extracted SDP body");
    }

    // Extract Via header (needed for 100 Trying response)
    let via_header = extract_via_header(message)?;
    eprintln!(
        "DEBUG:[UNIFIED_RECEIVER/INVITE] Extracted Via: {}",
        via_header
    );

    // Extract CSeq header (needed for 100 Trying response)
    let cseq_header = extract_cseq_header(message)?;
    eprintln!(
        "DEBUG:[UNIFIED_RECEIVER/INVITE] Extracted CSeq: {}",
        cseq_header
    );

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
                "DEBUG:[UNIFIED_RECEIVER/INVITE] Successfully handled incoming INVITE, created call: {}",
                call_id.as_str()
            );
            Ok(())
        }
        Err(e) => {
            eprintln!(
                "DEBUG:[UNIFIED_RECEIVER/INVITE] Failed to handle incoming INVITE: {}",
                e
            );
            Err(e)
        }
    }
}

/// Handle a SIP response message
async fn handle_response(
    message: &SipMessage,
    call_service: &Arc<Mutex<CallService>>,
) -> Result<(), SipError> {
    eprintln!("DEBUG:[UNIFIED_RECEIVER/RESPONSE] Processing response message");

    // Extract Call-ID header
    let call_id_header = extract_call_id_header(message)?;
    eprintln!(
        "DEBUG:[UNIFIED_RECEIVER/RESPONSE] Extracted Call-ID: {}",
        call_id_header
    );

    // Extract status code
    let status_code = extract_status_code(message)?;
    eprintln!(
        "DEBUG:[UNIFIED_RECEIVER/RESPONSE] Extracted status code: {}",
        status_code
    );

    // Extract SDP body (if present)
    let sdp_body = extract_sdp_body_receiver(message);
    if sdp_body.is_some() {
        eprintln!("DEBUG:[UNIFIED_RECEIVER/RESPONSE] Extracted SDP body");
    }

    // Find call by Call-ID header
    let call_id_opt = {
        let service = call_service.lock().await;
        service.find_call_by_call_id_header(&call_id_header).await
    };

    match call_id_opt {
        Some(call_id) => {
            eprintln!(
                "DEBUG:[UNIFIED_RECEIVER/RESPONSE] Matched Call-ID to call: {}",
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
                        "DEBUG:[UNIFIED_RECEIVER/RESPONSE] Successfully handled response for call: {}",
                        call_id.as_str()
                    );
                    Ok(())
                }
                Err(e) => {
                    eprintln!(
                        "DEBUG:[UNIFIED_RECEIVER/RESPONSE] Failed to handle response for call {}: {}",
                        call_id.as_str(),
                        e
                    );
                    Err(e)
                }
            }
        }
        None => {
            eprintln!(
                "DEBUG:[UNIFIED_RECEIVER/RESPONSE] No active call found for Call-ID: {}",
                call_id_header
            );
            // This might be a response for a call that already ended - not an error
            Ok(())
        }
    }
}
