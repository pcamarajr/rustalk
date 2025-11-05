/// Greets a person by name
/// 
/// # Arguments
/// 
/// * `name` - The name of the person to greet
/// 
/// # Returns
/// 
/// A greeting message as a String
#[tauri::command]
pub fn greet(name: String) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

