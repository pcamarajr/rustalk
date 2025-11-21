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
    answer_call, events::EventEmitter, get_input_device, get_output_device,
    get_registration_status, greet, hangup_call, hold_call, initiate_call, list_input_devices,
    list_output_devices, load_saved_credentials, mute_call, register_account, reject_call,
    set_input_device, set_output_device, unregister_account,
};
use domain::traits::CredentialStore;
use infrastructure::audio::create_audio_engine;
use infrastructure::sip::client::SipClient;
use infrastructure::sip::listener;
use infrastructure::sip::message_receiver;
#[cfg(target_os = "macos")]
use infrastructure::storage::KeychainCredentialStore;
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

            eprintln!("DEBUG:[SETUP] Creating SIP client for AuthService");
            // Initialize SIP client for AuthService using the runtime
            let client = rt
                .block_on(SipClient::new_udp_any())
                .map_err(|e| format!("Failed to create SIP client: {}", e))?;

            eprintln!("DEBUG:[SETUP] Creating SIP client for CallService");
            // Initialize SIP client for CallService using the runtime
            let call_client = rt
                .block_on(SipClient::new_udp_any())
                .map_err(|e| format!("Failed to create SIP client for calls: {}", e))?;

            eprintln!("DEBUG:[SETUP] SIP clients created, getting runtime handle");

            // Get runtime handle before moving runtime into thread
            let rt_handle = rt.handle().clone();

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

            eprintln!("DEBUG:[SETUP] Creating credential store");
            #[cfg(target_os = "macos")]
            let credential_store: Arc<dyn CredentialStore> =
                Arc::new(KeychainCredentialStore::new());
            #[cfg(not(target_os = "macos"))]
            {
                // For non-macOS platforms, we'll need to implement a platform-specific store
                // For now, this will cause a compilation error on non-macOS platforms
                // This is expected as KeychainCredentialStore is macOS-only
                compile_error!("Credential store not implemented for this platform");
            }
            eprintln!("DEBUG:[SETUP] Credential store created successfully");

            eprintln!("DEBUG:[SETUP] Creating event emitter");
            let event_emitter = EventEmitter::new(app.handle().clone());

            eprintln!("DEBUG:[SETUP] Creating AppState");
            // Wrap call_client in Arc<Mutex<>> so we can share it with message receiver
            let call_client = Arc::new(tokio::sync::Mutex::new(call_client));
            let call_client_for_receiver = Arc::clone(&call_client);

            let app_state = AppState::new(
                client,
                call_client,
                audio_service,
                credential_store,
                event_emitter,
            );

            // Get references for message receiver before managing app_state
            let call_service = Arc::clone(&app_state.call_service);

            app.manage(app_state);

            // Spawn message receiver background task
            eprintln!("DEBUG:[SETUP] Spawning message receiver background task");
            let call_client_for_receiver_clone = Arc::clone(&call_client_for_receiver);
            let call_service_for_receiver = Arc::clone(&call_service);
            rt_handle.spawn(async move {
                message_receiver::start_message_receiver(
                    call_client_for_receiver_clone,
                    call_service_for_receiver,
                )
                .await;
            });

            // Spawn INVITE listener background task
            eprintln!("DEBUG:[SETUP] Spawning INVITE listener background task");
            let call_client_for_listener = Arc::clone(&call_client_for_receiver);
            let call_service_for_listener = Arc::clone(&call_service);
            rt_handle.spawn(async move {
                listener::start_invite_listener(
                    call_client_for_listener,
                    call_service_for_listener,
                )
                .await;
            });

            eprintln!("DEBUG:[SETUP] Setup complete");
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            greet,
            register_account,
            get_registration_status,
            unregister_account,
            load_saved_credentials,
            list_input_devices,
            list_output_devices,
            get_input_device,
            get_output_device,
            set_input_device,
            set_output_device,
            initiate_call,
            answer_call,
            reject_call,
            hangup_call,
            mute_call,
            hold_call
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
