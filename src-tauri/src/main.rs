// Prevents additional console window on Windows in release mode
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tracing::info;
use tracing_subscriber;

fn main() {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info")),
        )
        .init();

    info!("Starting RUSTALK VoIP Application");

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![])
        .setup(|app| {
            info!("RUSTALK setup complete");

            #[cfg(target_os = "macos")]
            {
                use tauri::Manager;
                let window = app.get_window("main").unwrap();
                window.set_title("RUSTALK")?;
            }

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
