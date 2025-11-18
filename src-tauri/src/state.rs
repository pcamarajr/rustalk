// Application state for Tauri
// Holds shared state accessible by Tauri commands

use crate::infrastructure::sip::client::SipClient;
use crate::services::AuthService;
use std::sync::Arc;
use tokio::runtime::Handle;
use tokio::sync::Mutex;

/// Application state shared across Tauri commands
pub struct AppState {
    /// Authentication service for SIP account registration
    pub auth_service: Arc<Mutex<AuthService>>,
    /// Tokio runtime handle for spawning async tasks
    pub runtime_handle: Handle,
}

impl AppState {
    /// Create a new AppState with an AuthService
    ///
    /// # Arguments
    /// * `client` - SIP client instance for the AuthService
    /// * `runtime_handle` - Handle to the Tokio runtime
    pub fn new(client: SipClient, runtime_handle: Handle) -> Self {
        Self {
            auth_service: Arc::new(Mutex::new(AuthService::new(client))),
            runtime_handle,
        }
    }
}
