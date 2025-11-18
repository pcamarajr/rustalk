// Commands module - Tauri command handlers and validation
// This module contains all Tauri command handlers and their validation logic

pub mod validation;
pub mod auth;

pub use validation::*;
pub use auth::{register_account, get_registration_status, unregister_account};

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
