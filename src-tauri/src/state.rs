// Application state for Tauri
// Holds shared state accessible by Tauri commands

use crate::domain::traits::CredentialStore;
use crate::infrastructure::sip::client::SipClient;
use crate::services::{AudioService, AuthService, CallService};
use std::sync::Arc;
use tokio::sync::Mutex;

/// Application state shared across Tauri commands
pub struct AppState {
    /// Authentication service for SIP account registration
    pub auth_service: Arc<Mutex<AuthService>>,
    /// Audio service for device enumeration and selection
    pub audio_service: Arc<Mutex<AudioService>>,
    /// Call service for managing outbound calls
    pub call_service: Arc<Mutex<CallService>>,
    /// Credential store for secure credential persistence
    pub credential_store: Arc<dyn CredentialStore>,
}

impl AppState {
    /// Create a new AppState with an AuthService, AudioService, CallService, and CredentialStore
    ///
    /// # Arguments
    /// * `client` - SIP client instance for the AuthService
    /// * `call_client` - SIP client instance for the CallService (can be same as client)
    /// * `audio_service` - Audio service instance
    /// * `credential_store` - Credential store instance for secure credential persistence
    pub fn new(
        client: SipClient,
        call_client: SipClient,
        audio_service: AudioService,
        credential_store: Arc<dyn CredentialStore>,
    ) -> Self {
        // Clone credential store for AuthService
        let credential_store_for_auth = Arc::clone(&credential_store);
        let auth_service = Arc::new(Mutex::new(AuthService::new(
            client,
            credential_store_for_auth,
        )));

        // Clone auth_service for CallService
        let auth_service_for_call = Arc::clone(&auth_service);

        Self {
            auth_service,
            audio_service: Arc::new(Mutex::new(audio_service)),
            call_service: Arc::new(Mutex::new(CallService::new(
                call_client,
                auth_service_for_call,
            ))),
            credential_store,
        }
    }
}
