// Commands module - Tauri command handlers and validation
// This module contains all Tauri command handlers and their validation logic

pub mod audio;
pub mod auth;
pub mod validation;

pub use audio::{
    get_input_device, get_output_device, list_input_devices, list_output_devices, set_input_device,
    set_output_device,
};
pub use auth::{get_registration_status, register_account, unregister_account};
pub use validation::*;

use crate::domain::CommandError;

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
