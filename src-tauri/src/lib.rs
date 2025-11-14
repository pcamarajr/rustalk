// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/

// Domain layer
pub mod domain;

// Infrastructure layer
pub mod infrastructure;

// Commands module (IPC Boundary Layer)
pub mod commands;

use domain::CommandError;
use commands::validate_non_empty_string;

/// Example Tauri command demonstrating the validation pattern
///
/// # Arguments
///
/// * `name` - The name to greet (must be non-empty)
///
/// # Returns
///
/// * `Ok(String)` - A greeting message if validation passes
/// * `Err(CommandError)` - A validation error if the name is empty
#[tauri::command]
pub fn greet(name: &str) -> Result<String, CommandError> {
    validate_non_empty_string("name", name)?;
    Ok(format!("Hello, {}! You've been greeted from Rust!", name))
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
