// Call service - Orchestrates outbound call lifecycle

use crate::commands::events::EventEmitter;
use crate::domain::entities::call::{Call, CallId, CallState};
use crate::domain::entities::registration::RegistrationState;
use crate::domain::errors::SipError;
use crate::infrastructure::rtp::codec::G711Type;
use crate::infrastructure::rtp::session::{RtpSession, RtpSessionConfig};
use crate::infrastructure::sip::client::SipClient;
use crate::infrastructure::sip::invite::build_invite_with_sdp;
use crate::infrastructure::sip::sdp::{parse_sdp, CodecInfo};
use crate::services::auth_service::AuthService;
use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};
use tokio::sync::{mpsc, Mutex, RwLock};

/// RTP session data for a call
struct CallRtpSession {
    /// RTP session (wrapped in Mutex for mutable access)
    session: Arc<Mutex<RtpSession>>,
    /// Audio input channel (sends audio to RTP encoder)
    audio_tx: mpsc::Sender<Vec<i16>>,
    /// Audio output channel (receives decoded audio from RTP)
    /// Option allows taking ownership via take() method
    audio_rx: Option<mpsc::Receiver<Vec<i16>>>,
}

/// Call service managing outbound call lifecycle
pub struct CallService {
    /// Active calls (thread-safe)
    active_calls: Arc<RwLock<HashMap<CallId, Call>>>,
    /// RTP sessions for active calls
    rtp_sessions: Arc<RwLock<HashMap<CallId, CallRtpSession>>>,
    /// SIP client for sending messages
    sip_client: Arc<Mutex<SipClient>>,
    /// Reference to auth service for credentials and registration state
    auth_service: Arc<Mutex<AuthService>>,
    /// Local RTP ports for outbound calls (from SDP offer), stored per-call
    local_rtp_ports: Arc<RwLock<HashMap<CallId, u16>>>,
    /// Event emitter for sending events to frontend
    event_emitter: Option<EventEmitter>,
}

impl CallService {
    /// Create a new CallService with a SIP client and auth service
    ///
    /// # Arguments
    /// * `sip_client` - SIP client for sending/receiving messages (wrapped in Arc<Mutex<>>)
    /// * `auth_service` - Auth service for checking registration state
    /// * `event_emitter` - Optional event emitter for sending events to frontend
    pub fn new(
        sip_client: Arc<Mutex<SipClient>>,
        auth_service: Arc<Mutex<AuthService>>,
        event_emitter: Option<EventEmitter>,
    ) -> Self {
        Self {
            active_calls: Arc::new(RwLock::new(HashMap::new())),
            rtp_sessions: Arc::new(RwLock::new(HashMap::new())),
            sip_client,
            auth_service,
            local_rtp_ports: Arc::new(RwLock::new(HashMap::new())),
            event_emitter,
        }
    }

    /// Helper to convert CallState to string
    fn call_state_to_string(state: &CallState) -> String {
        match state {
            CallState::Idle => "idle".to_string(),
            CallState::Ringing => "ringing".to_string(),
            CallState::Connecting => "connecting".to_string(),
            CallState::Active => "active".to_string(),
            CallState::OnHold => "onHold".to_string(),
            CallState::Ended => "ended".to_string(),
        }
    }

    /// Helper to convert SystemTime to Unix timestamp in milliseconds
    fn system_time_to_timestamp(time: SystemTime) -> Option<u64> {
        time.duration_since(UNIX_EPOCH)
            .ok()
            .map(|d| d.as_millis() as u64)
    }

    /// Emit call state changed event if event emitter is available
    fn emit_state_changed(
        &self,
        call_id: &CallId,
        state: &CallState,
        start_time: Option<SystemTime>,
    ) {
        if let Some(emitter) = &self.event_emitter {
            let state_str = Self::call_state_to_string(state);
            let start_time_ts = start_time.and_then(Self::system_time_to_timestamp);
            emitter.emit_call_state_changed(call_id.as_str().to_string(), state_str, start_time_ts);
        }
    }

    /// Initiate an outbound call
    ///
    /// This method:
    /// 1. Validates registration is Registered
    /// 2. Creates Call entity
    /// 3. Transitions to Ringing state
    /// 4. Builds INVITE message with SDP
    /// 5. Sends INVITE via SIP client
    /// 6. Stores call in active_calls
    ///
    /// # Arguments
    /// * `number` - Remote phone number or URI
    /// * `local_address` - Local socket address for Via header
    /// * `server_addr` - SIP server address to send INVITE to
    /// * `contact_uri` - Contact header URI (e.g., "sip:user@192.168.1.100:5060")
    /// * `local_uri` - Local SIP URI (from credentials, e.g., "sip:alice@example.com")
    /// * `rtp_port` - RTP port for audio (must be even)
    /// * `username` - Username for SDP origin (typically from local_uri)
    ///
    /// # Returns
    /// `Ok(CallId)` if call was initiated successfully, `Err(SipError)` otherwise
    #[allow(clippy::too_many_arguments)]
    pub async fn initiate_outbound_call(
        &self,
        number: String,
        local_address: SocketAddr,
        server_addr: SocketAddr,
        contact_uri: String,
        local_uri: String,
        rtp_port: u16,
        username: String,
    ) -> Result<CallId, SipError> {
        eprintln!(
            "DEBUG:[CALL_SERVICE/INITIATE] Initiating outbound call to: {}",
            number
        );

        // Get registration state from auth_service
        let registration_state = {
            let auth = self.auth_service.lock().await;
            auth.get_registration_state().await
        };

        // Validate registration is Registered
        if !matches!(registration_state, RegistrationState::Registered) {
            eprintln!(
                "DEBUG:[CALL_SERVICE/INITIATE] Registration not registered, state: {:?}",
                registration_state
            );
            return Err(SipError::InvalidMessage {
                reason: format!(
                    "Cannot initiate call: registration state is {:?}",
                    registration_state
                ),
            });
        }

        eprintln!("DEBUG:[CALL_SERVICE/INITIATE] Registration validated, creating call entity");

        // Create Call entity
        let mut call = Call::new_outbound(number.clone());

        // Set local and remote URIs
        call.set_local_uri(local_uri.clone());
        let remote_uri = if number.starts_with("sip:") {
            number.clone()
        } else {
            format!("sip:{}", number)
        };
        call.set_remote_uri(remote_uri.clone());

        // Transition to Ringing state
        call.transition_to_ringing().map_err(|e| {
            eprintln!(
                "DEBUG:[CALL_SERVICE/INITIATE] Failed to transition to ringing: {}",
                e
            );
            e
        })?;

        eprintln!("DEBUG:[CALL_SERVICE/INITIATE] Call state set to ringing, building INVITE");

        // Build INVITE message with SDP
        let invite_bytes = build_invite_with_sdp(
            &remote_uri,
            &local_uri,
            &local_address,
            &contact_uri,
            rtp_port,
            &username,
            1, // CSeq starts at 1
        )
        .map_err(|e| {
            eprintln!(
                "DEBUG:[CALL_SERVICE/INITIATE] Failed to build INVITE: {}",
                e
            );
            e
        })?;

        // Extract Call-ID from the INVITE message for storage
        // Parse the message to extract Call-ID header
        let invite_str = String::from_utf8_lossy(&invite_bytes);
        if let Some(call_id_line) = invite_str
            .lines()
            .find(|line| line.starts_with("Call-ID:") || line.starts_with("i:"))
        {
            let call_id_value = call_id_line.split(':').nth(1).map(|s| s.trim().to_string());
            if let Some(call_id) = call_id_value {
                if !call_id.is_empty() {
                    call.set_call_id_header(call_id);
                } else {
                    eprintln!(
                        "DEBUG:[CALL_SERVICE/INITIATE] Warning: Call-ID header found but value is empty"
                    );
                }
            } else {
                eprintln!(
                    "DEBUG:[CALL_SERVICE/INITIATE] Warning: Failed to extract Call-ID value from header line"
                );
            }
        } else {
            eprintln!(
                "DEBUG:[CALL_SERVICE/INITIATE] Warning: Call-ID header not found in INVITE message"
            );
        }

        // Extract From tag from INVITE message
        if let Some(from_line) = invite_str
            .lines()
            .find(|line| line.starts_with("From:") || line.starts_with("f:"))
        {
            if let Some(tag_part) = from_line.split("tag=").nth(1) {
                let from_tag = tag_part.split(';').next().map(|s| s.trim().to_string());
                if let Some(tag) = from_tag {
                    if !tag.is_empty() {
                        call.set_from_tag(tag);
                    } else {
                        eprintln!(
                            "DEBUG:[CALL_SERVICE/INITIATE] Warning: From tag found but value is empty"
                        );
                    }
                } else {
                    eprintln!(
                        "DEBUG:[CALL_SERVICE/INITIATE] Warning: Failed to extract From tag value"
                    );
                }
            } else {
                eprintln!(
                    "DEBUG:[CALL_SERVICE/INITIATE] Warning: From tag not found in From header"
                );
            }
        } else {
            eprintln!(
                "DEBUG:[CALL_SERVICE/INITIATE] Warning: From header not found in INVITE message"
            );
        }

        eprintln!("DEBUG:[CALL_SERVICE/INITIATE] INVITE built, sending via SIP client");

        // Use server address as destination for the INVITE message
        let destination = server_addr;
        eprintln!(
            "DEBUG:[CALL_SERVICE/INITIATE] Sending INVITE to server: {}",
            destination
        );

        // Send INVITE via SIP client
        let client = self.sip_client.lock().await;
        client
            .send_bytes(&invite_bytes, &destination)
            .await
            .map_err(|e| {
                eprintln!("DEBUG:[CALL_SERVICE/INITIATE] Failed to send INVITE: {}", e);
                e
            })?;

        eprintln!("DEBUG:[CALL_SERVICE/INITIATE] INVITE sent, storing call");

        // Store call in active_calls
        let call_id = call.id().clone();
        let call_state = call.state().clone();
        let mut calls = self.active_calls.write().await;
        calls.insert(call_id.clone(), call);

        // Store local RTP port for this call
        {
            let mut local_ports = self.local_rtp_ports.write().await;
            local_ports.insert(call_id.clone(), rtp_port);
        }

        eprintln!(
            "DEBUG:[CALL_SERVICE/INITIATE] Call stored with ID: {}",
            call_id.as_str()
        );

        // Emit ringing state event
        self.emit_state_changed(&call_id, &call_state, None);

        Ok(call_id)
    }

    /// Get a call by ID
    ///
    /// # Arguments
    /// * `call_id` - Call identifier
    ///
    /// # Returns
    /// `Some(Call)` if call exists, `None` otherwise
    pub async fn get_call(&self, call_id: &CallId) -> Option<Call> {
        let calls = self.active_calls.read().await;
        calls.get(call_id).cloned()
    }

    /// Get call state by ID
    ///
    /// # Arguments
    /// * `call_id` - Call identifier
    ///
    /// # Returns
    /// `Some(CallState)` if call exists, `None` otherwise
    pub async fn get_call_state(&self, call_id: &CallId) -> Option<CallState> {
        let calls = self.active_calls.read().await;
        calls.get(call_id).map(|call| call.state().clone())
    }

    /// Find a call by Call-ID header value
    ///
    /// Searches active_calls HashMap for a call with matching Call-ID header value.
    /// This is used to match incoming SIP responses to active calls.
    ///
    /// # Arguments
    /// * `call_id_header` - Call-ID header value from SIP message
    ///
    /// # Returns
    /// `Some(CallId)` if call found, `None` otherwise
    pub async fn find_call_by_call_id_header(&self, call_id_header: &str) -> Option<CallId> {
        let calls = self.active_calls.read().await;
        for (call_id, call) in calls.iter() {
            if let Some(stored_call_id_header) = call.call_id_header() {
                if stored_call_id_header == call_id_header {
                    return Some(call_id.clone());
                }
            }
        }
        None
    }

    /// Handle INVITE response and update call state
    ///
    /// Updates call state based on SIP response status code:
    /// - 100 Trying: Stay in Ringing
    /// - 180 Ringing: Transition to Connecting
    /// - 200 OK: Transition to Active, set start_time, create RTP session
    /// - 4xx/5xx/6xx: Transition to Ended
    ///
    /// # Arguments
    /// * `call_id` - Call identifier
    /// * `status_code` - SIP response status code
    /// * `sdp_body` - Optional SDP body from response (required for 200 OK)
    ///
    /// # Returns
    /// `Ok(())` if state was updated successfully, `Err(SipError)` otherwise
    pub async fn handle_invite_response(
        &self,
        call_id: &CallId,
        status_code: u16,
        sdp_body: Option<&str>,
    ) -> Result<(), SipError> {
        eprintln!(
            "DEBUG:[CALL_SERVICE/HANDLE_RESPONSE] Handling response for call {}: status={}",
            call_id.as_str(),
            status_code
        );

        let mut calls = self.active_calls.write().await;
        let call = calls.get_mut(call_id).ok_or_else(|| {
            eprintln!(
                "DEBUG:[CALL_SERVICE/HANDLE_RESPONSE] Call not found: {}",
                call_id.as_str()
            );
            SipError::InvalidMessage {
                reason: format!("Call not found: {}", call_id.as_str()),
            }
        })?;

        match status_code {
            // 100 Trying: Stay in Ringing
            100 => {
                eprintln!("DEBUG:[CALL_SERVICE/HANDLE_RESPONSE] 100 Trying - staying in Ringing");
                Ok(())
            }
            // 180 Ringing: Transition to Connecting
            180 => {
                eprintln!(
                    "DEBUG:[CALL_SERVICE/HANDLE_RESPONSE] 180 Ringing - transitioning to Connecting"
                );
                call.transition_to_connecting().map_err(|e| {
                    eprintln!(
                        "DEBUG:[CALL_SERVICE/HANDLE_RESPONSE] Failed to transition to connecting: {}",
                        e
                    );
                    e
                })?;
                // Emit connecting state event
                let new_state = call.state().clone();
                self.emit_state_changed(call_id, &new_state, None);
                Ok(())
            }
            // 200 OK: Transition to Active and start RTP session
            200 => {
                eprintln!("DEBUG:[CALL_SERVICE/HANDLE_RESPONSE] 200 OK - transitioning to Active");
                call.transition_to_active().map_err(|e| {
                    eprintln!(
                        "DEBUG:[CALL_SERVICE/HANDLE_RESPONSE] Failed to transition to active: {}",
                        e
                    );
                    e
                })?;

                // Get start_time before emitting event
                let start_time = call.start_time();
                let new_state = call.state().clone();

                // Create RTP session if SDP is provided
                if let Some(sdp_str) = sdp_body {
                    eprintln!(
                        "DEBUG:[CALL_SERVICE/HANDLE_RESPONSE] Parsing SDP and creating RTP session"
                    );
                    if let Err(e) = self.create_rtp_session(call_id, sdp_str).await {
                        eprintln!(
                            "DEBUG:[CALL_SERVICE/HANDLE_RESPONSE] Failed to create RTP session: {}",
                            e
                        );
                        // Don't fail the call transition, but log the error
                        // In production, we might want to handle this differently
                    }
                } else {
                    eprintln!("DEBUG:[CALL_SERVICE/HANDLE_RESPONSE] Warning: 200 OK received without SDP body");
                }

                // Emit active state event with start_time
                self.emit_state_changed(call_id, &new_state, start_time);

                Ok(())
            }
            // 4xx/5xx/6xx: Transition to Ended
            code if (400..=699).contains(&code) => {
                eprintln!(
                    "DEBUG:[CALL_SERVICE/HANDLE_RESPONSE] Error response {} - transitioning to Ended",
                    code
                );
                call.transition_to_ended().map_err(|e| {
                    eprintln!(
                        "DEBUG:[CALL_SERVICE/HANDLE_RESPONSE] Failed to transition to ended: {}",
                        e
                    );
                    e
                })?;
                // Emit ended state event
                let new_state = call.state().clone();
                self.emit_state_changed(call_id, &new_state, None);
                Ok(())
            }
            // Unknown status code
            _ => {
                eprintln!(
                    "DEBUG:[CALL_SERVICE/HANDLE_RESPONSE] Unknown status code {} - transitioning to Ended",
                    status_code
                );
                call.transition_to_ended().map_err(|e| {
                    eprintln!(
                        "DEBUG:[CALL_SERVICE/HANDLE_RESPONSE] Failed to transition to ended: {}",
                        e
                    );
                    e
                })?;
                // Emit ended state event
                let new_state = call.state().clone();
                self.emit_state_changed(call_id, &new_state, None);
                Ok(())
            }
        }
    }

    /// End a call
    ///
    /// Transitions call to Ended state and sets end_time.
    /// Stops RTP session if active.
    /// The call remains in active_calls but is marked as ended.
    ///
    /// # Arguments
    /// * `call_id` - Call identifier
    ///
    /// # Returns
    /// `Ok(())` if call was ended successfully, `Err(SipError)` otherwise
    pub async fn end_call(&self, call_id: &CallId) -> Result<(), SipError> {
        eprintln!(
            "DEBUG:[CALL_SERVICE/END_CALL] Ending call: {}",
            call_id.as_str()
        );

        // Stop RTP session if active
        self.stop_rtp_session(call_id).await;

        let mut calls = self.active_calls.write().await;
        let call = calls.get_mut(call_id).ok_or_else(|| {
            eprintln!(
                "DEBUG:[CALL_SERVICE/END_CALL] Call not found: {}",
                call_id.as_str()
            );
            SipError::InvalidMessage {
                reason: format!("Call not found: {}", call_id.as_str()),
            }
        })?;

        call.transition_to_ended().map_err(|e| {
            eprintln!(
                "DEBUG:[CALL_SERVICE/END_CALL] Failed to transition to ended: {}",
                e
            );
            e
        })?;

        // Emit ended state event
        let new_state = call.state().clone();
        self.emit_state_changed(call_id, &new_state, None);

        eprintln!(
            "DEBUG:[CALL_SERVICE/END_CALL] Call ended successfully: {}",
            call_id.as_str()
        );

        Ok(())
    }

    /// Create RTP session for a call
    ///
    /// Parses SDP from 200 OK response and creates RTP session with appropriate codec.
    ///
    /// # Arguments
    /// * `call_id` - Call identifier
    /// * `sdp_str` - SDP body from 200 OK response
    ///
    /// # Returns
    /// `Ok(())` if RTP session was created successfully, `Err(SipError)` otherwise
    async fn create_rtp_session(&self, call_id: &CallId, sdp_str: &str) -> Result<(), SipError> {
        eprintln!(
            "DEBUG:[CALL_SERVICE/RTP] Creating RTP session for call: {}",
            call_id.as_str()
        );

        // Parse SDP
        let parsed_sdp = parse_sdp(sdp_str).map_err(|e| {
            eprintln!("DEBUG:[CALL_SERVICE/RTP] Failed to parse SDP: {}", e);
            SipError::from(e)
        })?;

        // Get local RTP port (from the offer we sent)
        let local_port = {
            let local_ports = self.local_rtp_ports.read().await;
            local_ports
                .get(call_id)
                .copied()
                .ok_or_else(|| SipError::InvalidMessage {
                    reason: format!("Local RTP port not set for call {}", call_id.as_str()),
                })?
        };

        // Select codec (prefer PCMU over PCMA)
        let codec_type = select_codec(&parsed_sdp.codecs)?;

        // Build remote address
        let remote_addr = SocketAddr::new(parsed_sdp.connection_ip, parsed_sdp.rtp_port);

        eprintln!(
            "DEBUG:[CALL_SERVICE/RTP] RTP config: local_port={}, remote={}, codec={:?}",
            local_port, remote_addr, codec_type
        );

        // Create RTP session config
        let rtp_config = RtpSessionConfig {
            local_port,
            remote_addr,
            codec_type,
            ssrc: None, // Generate random SSRC
        };

        // Create and start RTP session
        let mut rtp_session = RtpSession::new(rtp_config);
        let (audio_tx, audio_rx) = rtp_session.start().await.map_err(|e| {
            eprintln!(
                "DEBUG:[CALL_SERVICE/RTP] Failed to start RTP session: {}",
                e
            );
            SipError::from(e)
        })?;

        // Store RTP session
        let rtp_data = CallRtpSession {
            session: Arc::new(Mutex::new(rtp_session)),
            audio_tx,
            audio_rx: Some(audio_rx),
        };

        let mut rtp_sessions = self.rtp_sessions.write().await;
        rtp_sessions.insert(call_id.clone(), rtp_data);

        eprintln!("DEBUG:[CALL_SERVICE/RTP] RTP session created and started successfully");

        Ok(())
    }

    /// Stop RTP session for a call
    ///
    /// # Arguments
    /// * `call_id` - Call identifier
    async fn stop_rtp_session(&self, call_id: &CallId) {
        eprintln!(
            "DEBUG:[CALL_SERVICE/RTP] Stopping RTP session for call: {}",
            call_id.as_str()
        );

        let mut rtp_sessions = self.rtp_sessions.write().await;
        if let Some(rtp_data) = rtp_sessions.remove(call_id) {
            let mut session = rtp_data.session.lock().await;
            if let Err(e) = session.stop().await {
                eprintln!("DEBUG:[CALL_SERVICE/RTP] Error stopping RTP session: {}", e);
            } else {
                eprintln!("DEBUG:[CALL_SERVICE/RTP] RTP session stopped successfully");
            }
        }

        // Clean up RTP port for this call
        let mut local_ports = self.local_rtp_ports.write().await;
        local_ports.remove(call_id);
    }

    /// Get audio input channel for a call's RTP session
    ///
    /// # Arguments
    /// * `call_id` - Call identifier
    ///
    /// # Returns
    /// `Some(audio_tx)` if RTP session exists, `None` otherwise
    pub async fn get_rtp_audio_input(&self, call_id: &CallId) -> Option<mpsc::Sender<Vec<i16>>> {
        let rtp_sessions = self.rtp_sessions.read().await;
        rtp_sessions
            .get(call_id)
            .map(|rtp_data| rtp_data.audio_tx.clone())
    }

    /// Get audio output channel for a call's RTP session
    ///
    /// Note: This consumes the receiver from the RTP session.
    /// Only call this once per call.
    ///
    /// # Arguments
    /// * `call_id` - Call identifier
    ///
    /// # Returns
    /// `Some(audio_rx)` if RTP session exists, `None` otherwise
    pub async fn take_rtp_audio_output(
        &self,
        call_id: &CallId,
    ) -> Option<mpsc::Receiver<Vec<i16>>> {
        let mut rtp_sessions = self.rtp_sessions.write().await;
        rtp_sessions
            .get_mut(call_id)
            .and_then(|rtp_data| rtp_data.audio_rx.take())
    }

    /// Build and send 100 Trying provisional response
    ///
    /// RFC 3261 requires sending 100 Trying immediately upon receiving an INVITE.
    /// This response is sent to the source address of the INVITE.
    ///
    /// # Arguments
    /// * `call_id_header` - Call-ID header from INVITE
    /// * `from_header` - From header from INVITE
    /// * `to_header` - To header from INVITE
    /// * `via_header` - Via header from INVITE (first Via)
    /// * `cseq_header` - CSeq header from INVITE
    /// * `source_addr` - Source address to send response to
    ///
    /// # Returns
    /// `Ok(())` if response was sent successfully, `Err(SipError)` otherwise
    async fn send_100_trying(
        &self,
        call_id_header: &str,
        from_header: &str,
        to_header: &str,
        via_header: &str,
        cseq_header: &str,
        source_addr: SocketAddr,
    ) -> Result<(), SipError> {
        eprintln!("DEBUG:[CALL_SERVICE/100_TRYING] Building 100 Trying response");

        // Build 100 Trying response
        // Format: SIP/2.0 100 Trying\r\n
        let mut response = "SIP/2.0 100 Trying\r\n".to_string();

        // Copy Via header (required)
        response.push_str(&format!("Via: {}\r\n", via_header));

        // Copy From header (required)
        response.push_str(&format!("From: {}\r\n", from_header));

        // Copy To header (required, no tag added yet)
        response.push_str(&format!("To: {}\r\n", to_header));

        // Copy Call-ID header (required)
        response.push_str(&format!("Call-ID: {}\r\n", call_id_header));

        // Copy CSeq header (required)
        response.push_str(&format!("CSeq: {}\r\n", cseq_header));

        // Content-Length: 0 (no body)
        response.push_str("Content-Length: 0\r\n");

        // End of headers
        response.push_str("\r\n");

        // Parse to validate
        let response_bytes = response.into_bytes();
        crate::infrastructure::sip::parser::parse_message(&response_bytes)?;

        // Send via SIP client
        let client = self.sip_client.lock().await;
        client
            .send_bytes(&response_bytes, &source_addr)
            .await
            .map_err(|e| {
                eprintln!(
                    "DEBUG:[CALL_SERVICE/100_TRYING] Failed to send 100 Trying: {}",
                    e
                );
                e
            })?;

        eprintln!(
            "DEBUG:[CALL_SERVICE/100_TRYING] 100 Trying sent to {}",
            source_addr
        );

        Ok(())
    }

    /// Handle incoming INVITE request
    ///
    /// This method:
    /// 1. Validates registration is Registered
    /// 2. Creates inbound Call entity in Ringing state
    /// 3. Sends 100 Trying provisional response (RFC 3261 requirement)
    /// 4. Stores call in active_calls
    /// 5. Stores SDP offer for later answer generation (IN-3.2)
    ///
    /// # Arguments
    /// * `call_id_header` - Call-ID header from INVITE
    /// * `from_tag` - From tag from INVITE (optional)
    /// * `from_header` - From header from INVITE
    /// * `to_header` - To header from INVITE
    /// * `remote_uri` - Request-URI from INVITE (remote number)
    /// * `sdp_body` - Optional SDP body from INVITE
    /// * `via_header` - Via header from INVITE (first Via)
    /// * `cseq_header` - CSeq header from INVITE
    /// * `source_addr` - Source address of the INVITE
    ///
    /// # Returns
    /// `Ok(CallId)` if call was created successfully, `Err(SipError)` otherwise
    #[allow(clippy::too_many_arguments)]
    pub async fn handle_incoming_invite(
        &self,
        call_id_header: &str,
        from_tag: Option<&str>,
        from_header: &str,
        to_header: &str,
        remote_uri: &str,
        sdp_body: Option<&str>,
        via_header: &str,
        cseq_header: &str,
        source_addr: SocketAddr,
    ) -> Result<CallId, SipError> {
        eprintln!(
            "DEBUG:[CALL_SERVICE/INCOMING_INVITE] Handling incoming INVITE from: {}",
            remote_uri
        );

        // Get registration state from auth_service
        let registration_state = {
            let auth = self.auth_service.lock().await;
            auth.get_registration_state().await
        };

        // Validate registration is Registered
        if !matches!(registration_state, RegistrationState::Registered) {
            eprintln!(
                "DEBUG:[CALL_SERVICE/INCOMING_INVITE] Registration not registered, state: {:?}",
                registration_state
            );
            return Err(SipError::InvalidMessage {
                reason: format!(
                    "Cannot receive call: registration state is {:?}",
                    registration_state
                ),
            });
        }

        eprintln!(
            "DEBUG:[CALL_SERVICE/INCOMING_INVITE] Registration validated, creating inbound call"
        );

        // Extract remote number from From header or Request-URI
        // Try to extract from From header first (more reliable), fall back to Request-URI
        let remote_number = if let Some(uri_start) = from_header.find('<') {
            if let Some(uri_end) = from_header[uri_start + 1..].find('>') {
                from_header[uri_start + 1..uri_start + 1 + uri_end].to_string()
            } else {
                remote_uri.to_string()
            }
        } else {
            remote_uri.to_string()
        };

        // Create inbound Call entity
        let mut call = Call::new_inbound(
            remote_number.clone(),
            call_id_header.to_string(),
            from_tag.map(|t| t.to_string()),
        );

        // Set remote URI
        call.set_remote_uri(remote_uri.to_string());

        // Store SDP offer if present (for later answer generation in IN-3.2)
        if sdp_body.is_some() {
            eprintln!("DEBUG:[CALL_SERVICE/INCOMING_INVITE] SDP offer received, will be used for answer generation");
            // TODO: Store SDP offer in call entity or separate storage for IN-3.2
        }

        // Store call first so we can reference it
        let call_id = call.id().clone();
        let call_state = call.state().clone();
        let mut calls = self.active_calls.write().await;
        calls.insert(call_id.clone(), call);
        drop(calls); // Release lock before async operation

        // Send 100 Trying provisional response (RFC 3261 requirement)
        self.send_100_trying(
            call_id_header,
            from_header,
            to_header,
            via_header,
            cseq_header,
            source_addr,
        )
        .await
        .map_err(|e| {
            eprintln!(
                "DEBUG:[CALL_SERVICE/INCOMING_INVITE] Failed to send 100 Trying: {}",
                e
            );
            e
        })?;

        // Emit ringing state event
        self.emit_state_changed(&call_id, &call_state, None);

        eprintln!(
            "DEBUG:[CALL_SERVICE/INCOMING_INVITE] Inbound call created with ID: {}",
            call_id.as_str()
        );

        Ok(call_id)
    }
}

/// Select codec from SDP codec list
/// Prefers PCMU (payload type 0) over PCMA (payload type 8)
fn select_codec(codecs: &[CodecInfo]) -> Result<G711Type, SipError> {
    // Prefer PCMU
    if codecs
        .iter()
        .any(|c| c.codec_name == "PCMU" && c.clock_rate == 8000)
    {
        return Ok(G711Type::Pcmu);
    }

    // Fall back to PCMA
    if codecs
        .iter()
        .any(|c| c.codec_name == "PCMA" && c.clock_rate == 8000)
    {
        return Ok(G711Type::Pcma);
    }

    Err(SipError::InvalidMessage {
        reason: "No supported codec (PCMU/PCMA) found in SDP".to_string(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::traits::CredentialStore;
    use crate::infrastructure::sip::client::SipClient;
    use crate::services::auth_service::AuthService;
    use std::net::SocketAddr;

    // Mock credential store for testing
    struct MockCredentialStore;

    #[async_trait::async_trait]
    impl CredentialStore for MockCredentialStore {
        async fn save(
            &self,
            _key: &str,
            _credentials: &crate::domain::entities::credentials::Credentials,
        ) -> Result<(), crate::domain::errors::CredentialStoreError> {
            Ok(())
        }

        async fn load(
            &self,
            _key: &str,
        ) -> Result<
            Option<crate::domain::entities::credentials::Credentials>,
            crate::domain::errors::CredentialStoreError,
        > {
            Ok(None)
        }

        async fn delete(
            &self,
            _key: &str,
        ) -> Result<(), crate::domain::errors::CredentialStoreError> {
            Ok(())
        }

        async fn exists(
            &self,
            _key: &str,
        ) -> Result<bool, crate::domain::errors::CredentialStoreError> {
            Ok(false)
        }
    }

    async fn create_test_call_service() -> CallService {
        let client = SipClient::new_udp_any().await.unwrap();
        let client = Arc::new(Mutex::new(client));
        let credential_store = Arc::new(MockCredentialStore) as Arc<dyn CredentialStore>;
        let client_for_auth = SipClient::new_udp_any().await.unwrap();
        let auth_service = Arc::new(Mutex::new(AuthService::new(
            client_for_auth,
            credential_store,
        )));
        CallService::new(client, auth_service, None)
    }

    #[tokio::test]
    async fn test_initiate_outbound_call_not_registered() {
        let service = create_test_call_service().await;
        let local_addr: SocketAddr = "127.0.0.1:5060".parse().unwrap();
        let server_addr: SocketAddr = "127.0.0.1:5060".parse().unwrap();

        let result = service
            .initiate_outbound_call(
                "sip:bob@example.com".to_string(),
                local_addr,
                server_addr,
                "sip:alice@127.0.0.1:5060".to_string(),
                "sip:alice@example.com".to_string(),
                49172,
                "alice".to_string(),
            )
            .await;

        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("registration state"));
    }

    #[tokio::test]
    async fn test_get_call_not_found() {
        let service = create_test_call_service().await;
        let call_id = CallId::new("nonexistent".to_string());

        let call = service.get_call(&call_id).await;
        assert!(call.is_none());
    }

    #[tokio::test]
    async fn test_get_call_state_not_found() {
        let service = create_test_call_service().await;
        let call_id = CallId::new("nonexistent".to_string());

        let state = service.get_call_state(&call_id).await;
        assert!(state.is_none());
    }

    #[tokio::test]
    async fn test_handle_invite_response_call_not_found() {
        let service = create_test_call_service().await;
        let call_id = CallId::new("nonexistent".to_string());

        let result = service.handle_invite_response(&call_id, 200, None).await;
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("not found"));
    }

    #[tokio::test]
    async fn test_end_call_not_found() {
        let service = create_test_call_service().await;
        let call_id = CallId::new("nonexistent".to_string());

        let result = service.end_call(&call_id).await;
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("not found"));
    }

    #[tokio::test]
    async fn test_handle_incoming_invite_not_registered() {
        let service = create_test_call_service().await;
        let source_addr: SocketAddr = "127.0.0.1:5060".parse().unwrap();

        let result = service
            .handle_incoming_invite(
                "call-id-123",
                Some("from-tag-123"),
                "<sip:alice@example.com>;tag=from-tag-123",
                "<sip:bob@example.com>",
                "sip:bob@example.com",
                None,
                "SIP/2.0/UDP 127.0.0.1:5060;branch=z9hG4bK123",
                "1 INVITE",
                source_addr,
            )
            .await;

        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("registration state"));
    }

    // Note: Testing the registered case requires setting up registration state,
    // which is complex from unit tests due to private fields and async registration flow.
    // The registered case is better tested via integration tests where registration
    // is set up through the normal public API flow. The "not registered" test above
    // verifies the validation logic works correctly.
    // Testing with SDP also requires registered state, which is better tested via integration tests.
}
