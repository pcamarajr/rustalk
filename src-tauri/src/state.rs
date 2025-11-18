// Application state for Tauri
// Holds shared state accessible by Tauri commands

use crate::infrastructure::sip::client::SipClient;
use crate::services::AuthService;
use std::sync::Arc;
use tokio::sync::Mutex;

/// Application state shared across Tauri commands
pub struct AppState {
    /// Authentication service for SIP account registration
    pub auth_service: Arc<Mutex<AuthService>>,
}

impl AppState {
    /// Create a new AppState with an AuthService
    ///
    /// # Arguments
    /// * `client` - SIP client instance for the AuthService
    pub fn new(client: SipClient) -> Self {
        Self {
            auth_service: Arc::new(Mutex::new(AuthService::new(client))),
        }
    }
}

