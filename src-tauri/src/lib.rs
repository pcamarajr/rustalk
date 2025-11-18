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

use commands::{
    get_input_device, get_output_device, get_registration_status, greet, list_input_devices,
    list_output_devices, register_account, set_input_device, set_output_device, unregister_account,
};
use infrastructure::audio::create_audio_engine;
use infrastructure::sip::client::SipClient;
use services::AudioService;
use state::AppState;
use std::sync::Arc;
use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            eprintln!("DEBUG:[SETUP] Initializing Tokio runtime and SIP client");

            // Initialize Tokio runtime (must be kept alive for the lifetime of the app)
            // Use Runtime::new() which creates a multi-threaded runtime
            let rt = tokio::runtime::Runtime::new()
                .map_err(|e| format!("Failed to create tokio runtime: {}", e))?;

            eprintln!("DEBUG:[SETUP] Tokio runtime created successfully");

            eprintln!("DEBUG:[SETUP] Creating SIP client");
            // Initialize SIP client using the runtime
            let client = rt
                .block_on(SipClient::new_udp_any())
                .map_err(|e| format!("Failed to create SIP client: {}", e))?;

            eprintln!("DEBUG:[SETUP] SIP client created, spawning runtime keeper thread");

            // Spawn a background task to keep the runtime alive
            // The runtime will be dropped when this thread exits, so we keep it running
            std::thread::spawn(move || {
                eprintln!("DEBUG:[RUNTIME_KEEPER] Runtime keeper thread started");
                // Keep the runtime alive by running a long-lived future
                rt.block_on(async {
                    eprintln!(
                        "DEBUG:[RUNTIME_KEEPER] Runtime block_on started, keeping runtime alive"
                    );
                    // This future never completes, keeping the runtime alive
                    std::future::pending::<()>().await;
                });
            });

            eprintln!("DEBUG:[SETUP] Creating audio engine");
            let audio_engine = create_audio_engine()
                .map_err(|e| format!("Failed to create audio engine: {}", e))?;
            let audio_engine: Arc<dyn crate::domain::traits::audio_engine::AudioEngine> =
                Arc::from(audio_engine);
            eprintln!("DEBUG:[SETUP] Audio engine created successfully");

            eprintln!("DEBUG:[SETUP] Creating audio service");
            let audio_service = AudioService::new(audio_engine);
            eprintln!("DEBUG:[SETUP] Audio service created successfully");

            eprintln!("DEBUG:[SETUP] Creating AppState");
            let app_state = AppState::new(client, audio_service);

            app.manage(app_state);
            eprintln!("DEBUG:[SETUP] Setup complete");
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            greet,
            register_account,
            get_registration_status,
            unregister_account,
            list_input_devices,
            list_output_devices,
            get_input_device,
            get_output_device,
            set_input_device,
            set_output_device
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
