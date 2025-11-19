// Auth service - Orchestrates SIP account registration lifecycle

use crate::domain::entities::credentials::Credentials;
use crate::domain::entities::registration::{Registration, RegistrationState};
use crate::domain::errors::SipError;
use crate::domain::traits::CredentialStore;
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
    /// Credential store for secure credential persistence
    credential_store: Arc<dyn CredentialStore>,
}

impl AuthService {
    /// Create a new AuthService with a SIP client and credential store
    ///
    /// # Arguments
    /// * `client` - SIP client for sending/receiving messages
    /// * `credential_store` - Credential store for secure credential persistence
    pub fn new(client: SipClient, credential_store: Arc<dyn CredentialStore>) -> Self {
        Self {
            registration: Arc::new(RwLock::new(Registration::new())),
            client: Arc::new(tokio::sync::Mutex::new(client)),
            server_addr: None,
            contact_uri: None,
            credential_store,
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
        eprintln!(
            "DEBUG:[AUTH_SERVICE/REGISTER] Starting registration for user: {}@{}",
            credentials.username, credentials.server
        );

        // Validate credentials
        credentials.validate().map_err(|e| {
            eprintln!(
                "DEBUG:[AUTH_SERVICE/REGISTER] Credential validation failed: {}",
                e
            );
            SipError::InvalidMessage {
                reason: format!("Invalid credentials: {}", e),
            }
        })?;

        eprintln!(
            "DEBUG:[AUTH_SERVICE/REGISTER] Credentials validated, server_addr={}, contact_uri={}",
            server_addr, contact_uri
        );

        // Store server address and contact URI
        self.server_addr = Some(server_addr);
        self.contact_uri = Some(contact_uri.clone());

        // Transition to Registering state
        {
            let mut reg = self.registration.write().await;
            reg.start_registering(credentials.clone()).map_err(|e| {
                eprintln!(
                    "DEBUG:[AUTH_SERVICE/REGISTER] Failed to transition to registering state: {}",
                    e
                );
                e
            })?;
        }

        eprintln!("DEBUG:[AUTH_SERVICE/REGISTER] State set to registering, calling register_with_challenge");

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

        eprintln!(
            "DEBUG:[AUTH_SERVICE/REGISTER] register_with_challenge completed, result: {:?}",
            result
                .as_ref()
                .map(|r| format!("Status: {}, Message: {}", r.status_code, r.message))
                .unwrap_or_else(|e| format!("Error: {}", e))
        );

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
                eprintln!("DEBUG:[AUTH_SERVICE/HANDLE_RESULT] Registration result: status={}, message={}, expires={:?}", 
                    reg_result.status_code, reg_result.message, reg_result.expires);

                if reg_result.status_code == 200 {
                    // Success - transition to Registered
                    eprintln!("DEBUG:[AUTH_SERVICE/HANDLE_RESULT] Registration successful, transitioning to Registered state");
                    reg.set_registered(reg_result.expires).map_err(|e| {
                        eprintln!(
                            "DEBUG:[AUTH_SERVICE/HANDLE_RESULT] Failed to set registered state: {}",
                            e
                        );
                        e
                    })?;

                    // Save credentials to Keychain after successful registration
                    if let Some(credentials) = reg.credentials() {
                        let credential_key =
                            format!("{}@{}", credentials.username, credentials.server);
                        eprintln!(
                            "DEBUG:[AUTH_SERVICE/SAVE_CREDENTIALS] Saving credentials with key: {}",
                            credential_key
                        );

                        // Clone credentials and key for async operation
                        let creds_clone = credentials.clone();
                        let key_clone = credential_key.clone();
                        let store = Arc::clone(&self.credential_store);

                        // Save credentials (non-blocking, errors are logged but don't fail registration)
                        tokio::spawn(async move {
                            match store.save(&key_clone, &creds_clone).await {
                                Ok(()) => {
                                    eprintln!("DEBUG:[AUTH_SERVICE/SAVE_CREDENTIALS] Credentials saved successfully");

                                    // Also save the default_account pointer
                                    let store_clone = Arc::clone(&store);
                                    let key_for_pointer = key_clone.clone();
                                    Self::save_default_account_pointer(
                                        store_clone,
                                        key_for_pointer,
                                    )
                                    .await;
                                }
                                Err(e) => {
                                    eprintln!("DEBUG:[AUTH_SERVICE/SAVE_CREDENTIALS] Failed to save credentials: {}", e);
                                    // Note: We don't fail registration if credential save fails
                                    // Registration succeeded, storage is secondary
                                }
                            }
                        });
                    } else {
                        eprintln!("DEBUG:[AUTH_SERVICE/SAVE_CREDENTIALS] No credentials available to save");
                    }

                    Ok(())
                } else {
                    // Error response - transition to Failed
                    let error_msg = format!(
                        "Registration failed: {} {}",
                        reg_result.status_code, reg_result.message
                    );
                    eprintln!(
                        "DEBUG:[AUTH_SERVICE/HANDLE_RESULT] Registration failed: {}",
                        error_msg
                    );
                    reg.set_failed(error_msg.clone()).map_err(|e| {
                        eprintln!(
                            "DEBUG:[AUTH_SERVICE/HANDLE_RESULT] Failed to set failed state: {}",
                            e
                        );
                        e
                    })?;
                    Err(SipError::InvalidMessage { reason: error_msg })
                }
            }
            Err(e) => {
                // Registration error - transition to Failed
                let error_msg = format!("Registration error: {}", e);
                eprintln!(
                    "DEBUG:[AUTH_SERVICE/HANDLE_RESULT] Registration error: {}",
                    error_msg
                );
                reg.set_failed(error_msg.clone()).map_err(|e| {
                    eprintln!(
                        "DEBUG:[AUTH_SERVICE/HANDLE_RESULT] Failed to set failed state: {}",
                        e
                    );
                    e
                })?;
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

    /// Save the default_account pointer to the credential store
    ///
    /// This helper function saves a pointer to the credential key in a special
    /// Credentials object stored under the "default_account" key. This allows
    /// the application to quickly find the default account credentials.
    ///
    /// # Arguments
    /// * `store` - The credential store to save to
    /// * `credential_key` - The key of the credentials to point to
    async fn save_default_account_pointer(store: Arc<dyn CredentialStore>, credential_key: String) {
        let default_account_key = "default_account".to_string();
        let default_account_creds = Credentials::new(
            "default".to_string(),
            0,
            crate::domain::entities::credentials::TransportProtocol::Udp,
            credential_key.clone(),
            "".to_string(), // Empty password for default account pointer
        );

        match store
            .save(&default_account_key, &default_account_creds)
            .await
        {
            Ok(()) => {
                eprintln!("DEBUG:[AUTH_SERVICE/SAVE_CREDENTIALS] Default account pointer saved successfully");
            }
            Err(e) => {
                eprintln!("DEBUG:[AUTH_SERVICE/SAVE_CREDENTIALS] Failed to save default account pointer: {}", e);
            }
        }
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
            &self.credential_store,
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
        credential_store: &Arc<dyn CredentialStore>,
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

                    // Save credentials to Keychain after successful refresh
                    if let Some(credentials) = reg.credentials() {
                        let credential_key =
                            format!("{}@{}", credentials.username, credentials.server);
                        eprintln!(
                            "DEBUG:[AUTH_SERVICE/SAVE_CREDENTIALS] Saving credentials after refresh with key: {}",
                            credential_key
                        );

                        // Clone credentials and key for async operation
                        let creds_clone = credentials.clone();
                        let key_clone = credential_key.clone();
                        let store = Arc::clone(credential_store);

                        // Save credentials (non-blocking, errors are logged but don't fail registration)
                        tokio::spawn(async move {
                            match store.save(&key_clone, &creds_clone).await {
                                Ok(()) => {
                                    eprintln!("DEBUG:[AUTH_SERVICE/SAVE_CREDENTIALS] Credentials saved successfully after refresh");

                                    // Also save the default_account pointer
                                    let store_clone = Arc::clone(&store);
                                    let key_for_pointer = key_clone.clone();
                                    Self::save_default_account_pointer(
                                        store_clone,
                                        key_for_pointer,
                                    )
                                    .await;
                                }
                                Err(e) => {
                                    eprintln!("DEBUG:[AUTH_SERVICE/SAVE_CREDENTIALS] Failed to save credentials after refresh: {}", e);
                                    // Note: We don't fail registration if credential save fails
                                    // Registration succeeded, storage is secondary
                                }
                            }
                        });
                    } else {
                        eprintln!("DEBUG:[AUTH_SERVICE/SAVE_CREDENTIALS] No credentials available to save after refresh");
                    }

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

    /// Get credentials if registered
    ///
    /// # Returns
    /// * `Some(Credentials)` if registered and credentials are available
    /// * `None` if not registered or credentials not available
    pub async fn get_credentials(&self) -> Option<Credentials> {
        let reg = self.registration.read().await;
        reg.credentials().cloned()
    }

    /// Get server address if registered
    ///
    /// # Returns
    /// * `Some(SocketAddr)` if registered and server address is available
    /// * `None` if not registered or server address not available
    pub fn get_server_address(&self) -> Option<SocketAddr> {
        self.server_addr
    }

    /// Get contact URI if registered
    ///
    /// # Returns
    /// * `Some(String)` if registered and contact URI is available
    /// * `None` if not registered or contact URI not available
    pub fn get_contact_uri(&self) -> Option<String> {
        self.contact_uri.clone()
    }

    /// Get local address from SIP client
    ///
    /// # Returns
    /// * `SocketAddr` - Local address the SIP client is bound to
    pub async fn get_local_address(&self) -> Result<SocketAddr, SipError> {
        let client = self.client.lock().await;
        Ok(client.local_address())
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
        let credential_store = Arc::clone(&self.credential_store);

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
                    &credential_store,
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
    use crate::domain::errors::CredentialStoreError;
    use crate::domain::traits::CredentialStore;
    use async_trait::async_trait;
    use std::collections::HashMap;
    use std::sync::Arc;
    use tokio::sync::Mutex;

    fn create_test_credentials() -> Credentials {
        Credentials::new(
            "sip.example.com".to_string(),
            5060,
            TransportProtocol::Udp,
            "user1".to_string(),
            "password123".to_string(),
        )
    }

    // Mock credential store for testing
    struct MockCredentialStore {
        storage: Arc<Mutex<HashMap<String, Credentials>>>,
    }

    impl MockCredentialStore {
        fn new() -> Self {
            Self {
                storage: Arc::new(Mutex::new(HashMap::new())),
            }
        }
    }

    #[async_trait]
    impl CredentialStore for MockCredentialStore {
        async fn save(
            &self,
            key: &str,
            credentials: &Credentials,
        ) -> Result<(), CredentialStoreError> {
            let mut storage = self.storage.lock().await;
            storage.insert(key.to_string(), credentials.clone());
            Ok(())
        }

        async fn load(&self, key: &str) -> Result<Option<Credentials>, CredentialStoreError> {
            let storage = self.storage.lock().await;
            Ok(storage.get(key).cloned())
        }

        async fn delete(&self, key: &str) -> Result<(), CredentialStoreError> {
            let mut storage = self.storage.lock().await;
            storage.remove(key);
            Ok(())
        }

        async fn exists(&self, key: &str) -> Result<bool, CredentialStoreError> {
            let storage = self.storage.lock().await;
            Ok(storage.contains_key(key))
        }
    }

    fn create_mock_credential_store() -> Arc<dyn CredentialStore> {
        Arc::new(MockCredentialStore::new())
    }

    #[tokio::test]
    async fn test_new_auth_service() {
        let client = SipClient::new_udp_any().await.unwrap();
        let credential_store = create_mock_credential_store();
        let service = AuthService::new(client, credential_store);
        let state = service.get_registration_state().await;
        assert!(matches!(state, RegistrationState::Unregistered));
    }

    #[tokio::test]
    async fn test_get_registration_state() {
        let client = SipClient::new_udp_any().await.unwrap();
        let credential_store = create_mock_credential_store();
        let service = AuthService::new(client, credential_store);
        let state = service.get_registration_state().await;
        assert!(matches!(state, RegistrationState::Unregistered));
    }

    #[tokio::test]
    async fn test_unregister() {
        let client = SipClient::new_udp_any().await.unwrap();
        let credential_store = create_mock_credential_store();
        let service = AuthService::new(client, credential_store);
        assert!(service.unregister().await.is_ok());
        let state = service.get_registration_state().await;
        assert!(matches!(state, RegistrationState::Unregistered));
    }

    #[tokio::test]
    async fn test_handle_registration_result_success() {
        let client = SipClient::new_udp_any().await.unwrap();
        let credential_store = create_mock_credential_store();
        let service = AuthService::new(client, credential_store);

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

        // Give time for async credential save to complete
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
    }

    #[tokio::test]
    async fn test_handle_registration_result_failure() {
        let client = SipClient::new_udp_any().await.unwrap();
        let credential_store = create_mock_credential_store();
        let service = AuthService::new(client, credential_store);

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
        let credential_store = create_mock_credential_store();
        let service = AuthService::new(client, credential_store);

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
        let credential_store = create_mock_credential_store();
        let mut service = AuthService::new(client, credential_store);

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

    #[tokio::test]
    async fn test_save_credentials_after_successful_registration() {
        let client = SipClient::new_udp_any().await.unwrap();
        let mock_store = MockCredentialStore::new();
        let credential_store: Arc<dyn CredentialStore> = Arc::new(mock_store);
        let credential_store_clone = Arc::clone(&credential_store);
        let service = AuthService::new(client, credential_store);

        let creds = create_test_credentials();
        let expected_key = format!("{}@{}", creds.username, creds.server);

        // First transition to Registering
        {
            let mut reg = service.registration.write().await;
            reg.start_registering(creds.clone()).unwrap();
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

        // Give time for async credential save to complete
        tokio::time::sleep(tokio::time::Duration::from_millis(200)).await;

        // Verify credentials were saved
        let saved_creds = credential_store_clone.load(&expected_key).await.unwrap();
        assert!(
            saved_creds.is_some(),
            "Credentials should be saved after successful registration"
        );
        assert_eq!(
            saved_creds.unwrap(),
            creds,
            "Saved credentials should match original"
        );
    }
}
