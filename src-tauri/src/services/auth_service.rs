// Auth service - Orchestrates SIP account registration lifecycle

use crate::domain::entities::credentials::Credentials;
use crate::domain::entities::registration::{Registration, RegistrationState};
use crate::domain::errors::SipError;
use crate::infrastructure::sip::client::SipClient;
use crate::infrastructure::sip::registration::{register_with_challenge, RegistrationResult};
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::sync::RwLock;
use tokio::time::Duration;

/// Auth service managing SIP account registration state and lifecycle
pub struct AuthService {
    /// Registration state (thread-safe)
    registration: Arc<RwLock<Registration>>,
    /// SIP client for communication
    client: Arc<tokio::sync::Mutex<SipClient>>,
    /// Server address for registration
    server_addr: Option<SocketAddr>,
    /// Contact URI for registration
    contact_uri: Option<String>,
}

impl AuthService {
    /// Create a new AuthService with a SIP client
    ///
    /// # Arguments
    /// * `client` - SIP client for sending/receiving messages
    pub fn new(client: SipClient) -> Self {
        Self {
            registration: Arc::new(RwLock::new(Registration::new())),
            client: Arc::new(tokio::sync::Mutex::new(client)),
            server_addr: None,
            contact_uri: None,
        }
    }

    /// Get current registration state
    pub async fn get_registration_state(&self) -> RegistrationState {
        let reg = self.registration.read().await;
        reg.state().clone()
    }

    /// Start registration flow
    ///
    /// This method:
    /// 1. Validates credentials
    /// 2. Transitions state to Registering
    /// 3. Calls the registration infrastructure function
    /// 4. Updates state based on result
    ///
    /// # Arguments
    /// * `credentials` - SIP account credentials
    /// * `server_addr` - Server socket address
    /// * `contact_uri` - Contact URI for registration (e.g., "sip:user@192.168.1.100:5060")
    /// * `expires` - Registration expiration time in seconds (default: 3600)
    ///
    /// # Returns
    /// `Ok(())` if registration was initiated successfully, `Err(SipError)` otherwise
    pub async fn register(
        &mut self,
        credentials: Credentials,
        server_addr: SocketAddr,
        contact_uri: String,
        expires: u32,
    ) -> Result<(), SipError> {
        // Validate credentials
        credentials
            .validate()
            .map_err(|e| SipError::InvalidMessage {
                reason: format!("Invalid credentials: {}", e),
            })?;

        // Store server address and contact URI
        self.server_addr = Some(server_addr);
        self.contact_uri = Some(contact_uri.clone());

        // Transition to Registering state
        {
            let mut reg = self.registration.write().await;
            reg.start_registering(credentials.clone())?;
        }

        // Perform registration
        let mut client = self.client.lock().await;
        let result = register_with_challenge(
            &mut client,
            &credentials,
            &server_addr,
            &contact_uri,
            expires,
        )
        .await;

        // Handle registration result
        self.handle_registration_result(result).await
    }

    /// Handle registration result and update state
    ///
    /// # Arguments
    /// * `result` - Result from registration attempt
    ///
    /// # Returns
    /// `Ok(())` if state was updated successfully, `Err(SipError)` otherwise
    pub async fn handle_registration_result(
        &self,
        result: Result<RegistrationResult, SipError>,
    ) -> Result<(), SipError> {
        let mut reg = self.registration.write().await;

        match result {
            Ok(reg_result) => {
                if reg_result.status_code == 200 {
                    // Success - transition to Registered
                    reg.set_registered(reg_result.expires)?;
                    Ok(())
                } else {
                    // Error response - transition to Failed
                    let error_msg = format!(
                        "Registration failed: {} {}",
                        reg_result.status_code, reg_result.message
                    );
                    reg.set_failed(error_msg.clone())?;
                    Err(SipError::InvalidMessage { reason: error_msg })
                }
            }
            Err(e) => {
                // Registration error - transition to Failed
                let error_msg = format!("Registration error: {}", e);
                reg.set_failed(error_msg.clone())?;
                Err(SipError::InvalidMessage { reason: error_msg })
            }
        }
    }

    /// Unregister account
    ///
    /// Transitions state from Registered to Unregistered.
    /// Note: This does not send an unregister SIP message, it only updates state.
    /// For full unregistration, you would need to send a REGISTER with expires=0.
    ///
    /// # Returns
    /// `Ok(())` if unregistration was successful, `Err(SipError)` otherwise
    pub async fn unregister(&self) -> Result<(), SipError> {
        let mut reg = self.registration.write().await;
        reg.set_unregistered()
    }

    /// Refresh registration if expired
    ///
    /// Checks if registration is expired and attempts to re-register if needed.
    /// This method should be called periodically to maintain registration.
    ///
    /// # Returns
    /// `Ok(true)` if re-registration was attempted, `Ok(false)` if not needed, `Err(SipError)` if re-registration failed
    pub async fn refresh_registration(&mut self) -> Result<bool, SipError> {
        Self::refresh_registration_internal(
            &self.registration,
            &self.client,
            self.server_addr,
            self.contact_uri.as_ref(),
        )
        .await
    }

    /// Internal helper to refresh registration without requiring &mut self
    ///
    /// This allows the refresh logic to be called from background tasks without
    /// creating unnecessary service instances.
    async fn refresh_registration_internal(
        registration: &Arc<RwLock<Registration>>,
        client: &Arc<tokio::sync::Mutex<SipClient>>,
        server_addr: Option<SocketAddr>,
        contact_uri: Option<&String>,
    ) -> Result<bool, SipError> {
        // Check if registration is expired
        let should_refresh = {
            let mut reg = registration.write().await;
            reg.check_expiration();
            matches!(reg.state(), RegistrationState::Expired)
        };

        if !should_refresh {
            return Ok(false);
        }

        // Get credentials and server info
        let (credentials, server_addr, contact_uri) = {
            let reg = registration.read().await;
            let creds = reg.credentials().ok_or_else(|| SipError::InvalidMessage {
                reason: "No credentials available for refresh".to_string(),
            })?;
            let server = server_addr.ok_or_else(|| SipError::InvalidMessage {
                reason: "No server address available for refresh".to_string(),
            })?;
            let contact = contact_uri
                .ok_or_else(|| SipError::InvalidMessage {
                    reason: "No contact URI available for refresh".to_string(),
                })?
                .clone();
            (creds.clone(), server, contact)
        };

        // Validate credentials
        credentials
            .validate()
            .map_err(|e| SipError::InvalidMessage {
                reason: format!("Invalid credentials: {}", e),
            })?;

        // Transition to Registering state
        {
            let mut reg = registration.write().await;
            reg.start_registering(credentials.clone())?;
        }

        // Perform registration
        let mut client_guard = client.lock().await;
        let result = register_with_challenge(
            &mut client_guard,
            &credentials,
            &server_addr,
            &contact_uri,
            3600,
        )
        .await;

        // Handle registration result
        let mut reg = registration.write().await;
        match result {
            Ok(reg_result) => {
                if reg_result.status_code == 200 {
                    // Success - transition to Registered
                    reg.set_registered(reg_result.expires)?;
                    Ok(true)
                } else {
                    // Error response - transition to Failed
                    let error_msg = format!(
                        "Registration failed: {} {}",
                        reg_result.status_code, reg_result.message
                    );
                    reg.set_failed(error_msg.clone())?;
                    Err(SipError::InvalidMessage { reason: error_msg })
                }
            }
            Err(e) => {
                // Registration error - transition to Failed
                let error_msg = format!("Registration error: {}", e);
                reg.set_failed(error_msg.clone())?;
                Err(SipError::InvalidMessage { reason: error_msg })
            }
        }
    }

    /// Start background task to monitor registration expiration
    ///
    /// This spawns a background task that periodically checks for expiration
    /// and attempts to refresh the registration if needed.
    ///
    /// # Arguments
    /// * `check_interval_secs` - How often to check for expiration (default: 60 seconds)
    ///
    /// # Returns
    /// A handle to stop the background task
    pub fn start_expiration_monitor(
        &self,
        check_interval_secs: u64,
    ) -> tokio::task::JoinHandle<()> {
        let registration = Arc::clone(&self.registration);
        let client = Arc::clone(&self.client);
        let server_addr = self.server_addr;
        let contact_uri = self.contact_uri.clone();

        tokio::spawn(async move {
            let mut interval = tokio::time::interval(Duration::from_secs(check_interval_secs));
            loop {
                interval.tick().await;

                // Check expiration
                {
                    let mut reg = registration.write().await;
                    reg.check_expiration();
                }

                // Attempt refresh if expired
                if let Err(e) = AuthService::refresh_registration_internal(
                    &registration,
                    &client,
                    server_addr,
                    contact_uri.as_ref(),
                )
                .await
                {
                    eprintln!("DEBUG:[AUTH_SERVICE/EXPIRATION_MONITOR] Failed to refresh registration: {}", e);
                }
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::entities::credentials::{Credentials, TransportProtocol};

    fn create_test_credentials() -> Credentials {
        Credentials::new(
            "sip.example.com".to_string(),
            5060,
            TransportProtocol::Udp,
            "user1".to_string(),
            "password123".to_string(),
        )
    }

    #[tokio::test]
    async fn test_new_auth_service() {
        let client = SipClient::new_udp_any().await.unwrap();
        let service = AuthService::new(client);
        let state = service.get_registration_state().await;
        assert!(matches!(state, RegistrationState::Unregistered));
    }

    #[tokio::test]
    async fn test_get_registration_state() {
        let client = SipClient::new_udp_any().await.unwrap();
        let service = AuthService::new(client);
        let state = service.get_registration_state().await;
        assert!(matches!(state, RegistrationState::Unregistered));
    }

    #[tokio::test]
    async fn test_unregister() {
        let client = SipClient::new_udp_any().await.unwrap();
        let service = AuthService::new(client);
        assert!(service.unregister().await.is_ok());
        let state = service.get_registration_state().await;
        assert!(matches!(state, RegistrationState::Unregistered));
    }

    #[tokio::test]
    async fn test_handle_registration_result_success() {
        let client = SipClient::new_udp_any().await.unwrap();
        let service = AuthService::new(client);

        // First transition to Registering
        {
            let mut reg = service.registration.write().await;
            let creds = create_test_credentials();
            reg.start_registering(creds).unwrap();
        }

        // Handle successful result
        let result = RegistrationResult {
            status_code: 200,
            expires: Some(3600),
            message: "OK".to_string(),
        };

        assert!(service.handle_registration_result(Ok(result)).await.is_ok());
        let state = service.get_registration_state().await;
        assert!(matches!(state, RegistrationState::Registered));
    }

    #[tokio::test]
    async fn test_handle_registration_result_failure() {
        let client = SipClient::new_udp_any().await.unwrap();
        let service = AuthService::new(client);

        // First transition to Registering
        {
            let mut reg = service.registration.write().await;
            let creds = create_test_credentials();
            reg.start_registering(creds).unwrap();
        }

        // Handle failed result
        let result = RegistrationResult {
            status_code: 401,
            expires: None,
            message: "Unauthorized".to_string(),
        };

        assert!(service
            .handle_registration_result(Ok(result))
            .await
            .is_err());
        let state = service.get_registration_state().await;
        assert!(matches!(state, RegistrationState::Failed(_)));
    }

    #[tokio::test]
    async fn test_handle_registration_result_error() {
        let client = SipClient::new_udp_any().await.unwrap();
        let service = AuthService::new(client);

        // First transition to Registering
        {
            let mut reg = service.registration.write().await;
            let creds = create_test_credentials();
            reg.start_registering(creds).unwrap();
        }

        // Handle error result
        let error = SipError::InvalidMessage {
            reason: "Network error".to_string(),
        };

        assert!(service
            .handle_registration_result(Err(error))
            .await
            .is_err());
        let state = service.get_registration_state().await;
        assert!(matches!(state, RegistrationState::Failed(_)));
    }

    #[tokio::test]
    async fn test_refresh_registration_not_expired() {
        let client = SipClient::new_udp_any().await.unwrap();
        let mut service = AuthService::new(client);

        // Set to Registered state
        {
            let mut reg = service.registration.write().await;
            let creds = create_test_credentials();
            reg.start_registering(creds).unwrap();
            reg.set_registered(Some(3600)).unwrap();
        }

        // Refresh should return false (not needed)
        let result = service.refresh_registration().await;
        assert!(result.is_ok());
        assert!(!result.unwrap());
    }
}
