// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/

// Domain layer
pub mod domain;

// Infrastructure layer
pub mod infrastructure;

// Commands module (IPC Boundary Layer)
pub mod commands;

use commands::greet;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
