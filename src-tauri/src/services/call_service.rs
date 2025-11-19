// Call service - Orchestrates outbound call lifecycle

use crate::domain::entities::call::{Call, CallId, CallState};
use crate::domain::entities::registration::RegistrationState;
use crate::domain::errors::SipError;
use crate::infrastructure::sip::client::SipClient;
use crate::infrastructure::sip::invite::build_invite_with_sdp;
use crate::services::auth_service::AuthService;
use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::sync::{Mutex, RwLock};

/// Call service managing outbound call lifecycle
pub struct CallService {
    /// Active calls (thread-safe)
    active_calls: Arc<RwLock<HashMap<CallId, Call>>>,
    /// SIP client for sending messages
    sip_client: Arc<Mutex<SipClient>>,
    /// Reference to auth service for credentials and registration state
    auth_service: Arc<Mutex<AuthService>>,
}

impl CallService {
    /// Create a new CallService with a SIP client and auth service
    ///
    /// # Arguments
    /// * `sip_client` - SIP client for sending/receiving messages
    /// * `auth_service` - Auth service for checking registration state
    pub fn new(sip_client: SipClient, auth_service: Arc<Mutex<AuthService>>) -> Self {
        Self {
            active_calls: Arc::new(RwLock::new(HashMap::new())),
            sip_client: Arc::new(Mutex::new(sip_client)),
            auth_service,
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
    /// * `contact_uri` - Contact header URI (e.g., "sip:user@192.168.1.100:5060")
    /// * `local_uri` - Local SIP URI (from credentials, e.g., "sip:alice@example.com")
    /// * `rtp_port` - RTP port for audio (must be even)
    /// * `username` - Username for SDP origin (typically from local_uri)
    ///
    /// # Returns
    /// `Ok(CallId)` if call was initiated successfully, `Err(SipError)` otherwise
    pub async fn initiate_outbound_call(
        &self,
        number: String,
        local_address: SocketAddr,
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
            let call_id_value = call_id_line
                .split(':')
                .nth(1)
                .map(|s| s.trim().to_string())
                .unwrap_or_default();
            call.set_call_id_header(call_id_value);
        }

        // Extract From tag from INVITE message
        if let Some(from_line) = invite_str
            .lines()
            .find(|line| line.starts_with("From:") || line.starts_with("f:"))
        {
            if let Some(tag_part) = from_line.split("tag=").nth(1) {
                let from_tag = tag_part
                    .split(';')
                    .next()
                    .map(|s| s.trim().to_string())
                    .unwrap_or_default();
                call.set_from_tag(from_tag);
            }
        }

        eprintln!("DEBUG:[CALL_SERVICE/INITIATE] INVITE built, sending via SIP client");

        // Get server address from auth service (we'll need to construct it)
        // For now, we'll use the local_address as destination (this should be improved)
        // In a real implementation, we'd resolve the SIP server address
        let destination = local_address; // TODO: Get actual SIP server address

        // Send INVITE via SIP client
        let client = self.sip_client.lock().await;
        client.send_bytes(&invite_bytes, &destination).await.map_err(|e| {
            eprintln!(
                "DEBUG:[CALL_SERVICE/INITIATE] Failed to send INVITE: {}",
                e
            );
            e
        })?;

        eprintln!("DEBUG:[CALL_SERVICE/INITIATE] INVITE sent, storing call");

        // Store call in active_calls
        let call_id = call.id().clone();
        let mut calls = self.active_calls.write().await;
        calls.insert(call_id.clone(), call);

        eprintln!(
            "DEBUG:[CALL_SERVICE/INITIATE] Call stored with ID: {}",
            call_id.as_str()
        );

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

    /// Handle INVITE response and update call state
    ///
    /// Updates call state based on SIP response status code:
    /// - 100 Trying: Stay in Ringing
    /// - 180 Ringing: Transition to Connecting
    /// - 200 OK: Transition to Active, set start_time
    /// - 4xx/5xx/6xx: Transition to Ended
    ///
    /// # Arguments
    /// * `call_id` - Call identifier
    /// * `status_code` - SIP response status code
    ///
    /// # Returns
    /// `Ok(())` if state was updated successfully, `Err(SipError)` otherwise
    pub async fn handle_invite_response(
        &self,
        call_id: &CallId,
        status_code: u16,
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
                eprintln!(
                    "DEBUG:[CALL_SERVICE/HANDLE_RESPONSE] 100 Trying - staying in Ringing"
                );
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
                })
            }
            // 200 OK: Transition to Active
            200 => {
                eprintln!(
                    "DEBUG:[CALL_SERVICE/HANDLE_RESPONSE] 200 OK - transitioning to Active"
                );
                call.transition_to_active().map_err(|e| {
                    eprintln!(
                        "DEBUG:[CALL_SERVICE/HANDLE_RESPONSE] Failed to transition to active: {}",
                        e
                    );
                    e
                })
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
                })
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
                })
            }
        }
    }

    /// End a call
    ///
    /// Transitions call to Ended state and sets end_time.
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

        eprintln!(
            "DEBUG:[CALL_SERVICE/END_CALL] Call ended successfully: {}",
            call_id.as_str()
        );

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::infrastructure::sip::client::SipClient;
    use crate::services::auth_service::AuthService;
    use crate::domain::traits::CredentialStore;
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
        ) -> Result<Option<crate::domain::entities::credentials::Credentials>, crate::domain::errors::CredentialStoreError> {
            Ok(None)
        }

        async fn delete(&self, _key: &str) -> Result<(), crate::domain::errors::CredentialStoreError> {
            Ok(())
        }

        async fn exists(&self, _key: &str) -> Result<bool, crate::domain::errors::CredentialStoreError> {
            Ok(false)
        }
    }

    async fn create_test_call_service() -> CallService {
        let client = SipClient::new_udp_any().await.unwrap();
        let credential_store = Arc::new(MockCredentialStore) as Arc<dyn CredentialStore>;
        let client_for_auth = SipClient::new_udp_any().await.unwrap();
        let auth_service = Arc::new(Mutex::new(AuthService::new(client_for_auth, credential_store)));
        CallService::new(client, auth_service)
    }

    #[tokio::test]
    async fn test_initiate_outbound_call_not_registered() {
        let service = create_test_call_service().await;
        let local_addr: SocketAddr = "127.0.0.1:5060".parse().unwrap();

        let result = service
            .initiate_outbound_call(
                "sip:bob@example.com".to_string(),
                local_addr,
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

        let result = service.handle_invite_response(&call_id, 200).await;
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
}

