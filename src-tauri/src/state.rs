// Application state for Tauri
// Holds shared state accessible by Tauri commands

use crate::domain::traits::CredentialStore;
use crate::infrastructure::sip::client::SipClient;
use crate::services::{AudioService, AuthService};
use std::sync::Arc;
use tokio::sync::Mutex;

/// Application state shared across Tauri commands
pub struct AppState {
    /// Authentication service for SIP account registration
    pub auth_service: Arc<Mutex<AuthService>>,
    /// Audio service for device enumeration and selection
    pub audio_service: Arc<Mutex<AudioService>>,
    /// Credential store for secure credential persistence
    pub credential_store: Arc<dyn CredentialStore>,
}

impl AppState {
    /// Create a new AppState with an AuthService, AudioService, and CredentialStore
    ///
    /// # Arguments
    /// * `client` - SIP client instance for the AuthService
    /// * `audio_service` - Audio service instance
    /// * `credential_store` - Credential store instance for secure credential persistence
    pub fn new(
        client: SipClient,
        audio_service: AudioService,
        credential_store: Arc<dyn CredentialStore>,
    ) -> Self {
        // Clone credential store for AuthService
        let credential_store_for_auth = Arc::clone(&credential_store);
        Self {
            auth_service: Arc::new(Mutex::new(AuthService::new(
                client,
                credential_store_for_auth,
            ))),
            audio_service: Arc::new(Mutex::new(audio_service)),
            credential_store,
        }
    }
}
