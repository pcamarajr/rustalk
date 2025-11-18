// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/

// Domain layer
pub mod domain;

// Infrastructure layer
pub mod infrastructure;

// Services layer
pub mod services;

// Commands module (IPC Boundary Layer)
pub mod commands;

// Application state
pub mod state;

use commands::{get_registration_status, greet, register_account, unregister_account};
use infrastructure::sip::client::SipClient;
use state::AppState;
use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            // Initialize SIP client and AppState
            let rt = tokio::runtime::Runtime::new()
                .map_err(|e| format!("Failed to create tokio runtime: {}", e))?;
            let client = rt
                .block_on(SipClient::new_udp_any())
                .map_err(|e| format!("Failed to create SIP client: {}", e))?;
            let app_state = AppState::new(client);

            app.manage(app_state);
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            greet,
            register_account,
            get_registration_status,
            unregister_account
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
