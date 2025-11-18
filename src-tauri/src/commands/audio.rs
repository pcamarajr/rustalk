// Audio commands for device enumeration and selection

use crate::commands::validation::validate_non_empty_string;
use crate::domain::errors::CommandError;
use crate::domain::traits::audio_engine::AudioDevice;
use crate::state::AppState;
use tauri::State;

/// List all available input audio devices
///
/// # Returns
/// * `Ok(Vec<AudioDevice>)` - List of available input devices
/// * `Err(CommandError)` - Error enumerating devices
#[tauri::command]
pub async fn list_input_devices(
    state: State<'_, AppState>,
) -> Result<Vec<AudioDevice>, CommandError> {
    eprintln!("DEBUG:[AUDIO/COMMAND] list_input_devices called");
    let audio_service = state.audio_service.lock().await;
    let devices = audio_service
        .list_input_devices()
        .await
        .map_err(CommandError::from)?;
    eprintln!(
        "DEBUG:[AUDIO/COMMAND] list_input_devices returned {} devices",
        devices.len()
    );
    Ok(devices)
}

/// List all available output audio devices
///
/// # Returns
/// * `Ok(Vec<AudioDevice>)` - List of available output devices
/// * `Err(CommandError)` - Error enumerating devices
#[tauri::command]
pub async fn list_output_devices(
    state: State<'_, AppState>,
) -> Result<Vec<AudioDevice>, CommandError> {
    eprintln!("DEBUG:[AUDIO/COMMAND] list_output_devices called");
    let audio_service = state.audio_service.lock().await;
    let devices = audio_service
        .list_output_devices()
        .await
        .map_err(CommandError::from)?;
    eprintln!(
        "DEBUG:[AUDIO/COMMAND] list_output_devices returned {} devices",
        devices.len()
    );
    Ok(devices)
}

/// Get the currently selected input device
///
/// # Returns
/// * `Ok(Some(AudioDevice))` - Current input device
/// * `Ok(None)` - No input device selected
/// * `Err(CommandError)` - Error retrieving device
#[tauri::command]
pub async fn get_input_device(
    state: State<'_, AppState>,
) -> Result<Option<AudioDevice>, CommandError> {
    eprintln!("DEBUG:[AUDIO/COMMAND] get_input_device called");
    let audio_service = state.audio_service.lock().await;
    let device = audio_service
        .get_input_device()
        .await
        .map_err(CommandError::from)?;
    eprintln!(
        "DEBUG:[AUDIO/COMMAND] get_input_device returned: {:?}",
        device.as_ref().map(|d| &d.id)
    );
    Ok(device)
}

/// Get the currently selected output device
///
/// # Returns
/// * `Ok(Some(AudioDevice))` - Current output device
/// * `Ok(None)` - No output device selected
/// * `Err(CommandError)` - Error retrieving device
#[tauri::command]
pub async fn get_output_device(
    state: State<'_, AppState>,
) -> Result<Option<AudioDevice>, CommandError> {
    eprintln!("DEBUG:[AUDIO/COMMAND] get_output_device called");
    let audio_service = state.audio_service.lock().await;
    let device = audio_service
        .get_output_device()
        .await
        .map_err(CommandError::from)?;
    eprintln!(
        "DEBUG:[AUDIO/COMMAND] get_output_device returned: {:?}",
        device.as_ref().map(|d| &d.id)
    );
    Ok(device)
}

/// Set the active input device
///
/// # Arguments
/// * `device_id` - Unique identifier of the device to select
///
/// # Returns
/// * `Ok(String)` - Success message
/// * `Err(CommandError)` - Validation or service error
#[tauri::command]
pub async fn set_input_device(
    device_id: String,
    state: State<'_, AppState>,
) -> Result<String, CommandError> {
    eprintln!(
        "DEBUG:[AUDIO/COMMAND] set_input_device called with device_id: {}",
        device_id
    );
    validate_non_empty_string("device_id", &device_id)?;
    let audio_service = state.audio_service.lock().await;
    audio_service
        .set_input_device(&device_id)
        .await
        .map_err(CommandError::from)?;
    eprintln!(
        "DEBUG:[AUDIO/COMMAND] set_input_device succeeded for device_id: {}",
        device_id
    );
    Ok("Input device set successfully".to_string())
}

/// Set the active output device
///
/// # Arguments
/// * `device_id` - Unique identifier of the device to select
///
/// # Returns
/// * `Ok(String)` - Success message
/// * `Err(CommandError)` - Validation or service error
#[tauri::command]
pub async fn set_output_device(
    device_id: String,
    state: State<'_, AppState>,
) -> Result<String, CommandError> {
    eprintln!(
        "DEBUG:[AUDIO/COMMAND] set_output_device called with device_id: {}",
        device_id
    );
    validate_non_empty_string("device_id", &device_id)?;
    let audio_service = state.audio_service.lock().await;
    audio_service
        .set_output_device(&device_id)
        .await
        .map_err(CommandError::from)?;
    eprintln!(
        "DEBUG:[AUDIO/COMMAND] set_output_device succeeded for device_id: {}",
        device_id
    );
    Ok("Output device set successfully".to_string())
}
