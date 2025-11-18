// Integration tests for audio Tauri commands
// Tests error conversion and command structure

use rustalk_lib::domain::errors::{AudioEngineError, CommandError};
use rustalk_lib::domain::traits::audio_engine::{AudioDevice, AudioEngine};
use rustalk_lib::services::audio_service::AudioService;
use std::sync::Arc;

#[test]
fn test_audio_engine_error_to_command_error_conversion() {
    // Test DeviceNotFound conversion
    let audio_error = AudioEngineError::DeviceNotFound {
        device_id: "invalid-device".to_string(),
    };
    let command_error: CommandError = audio_error.into();
    match command_error {
        CommandError::InvalidArgument { argument, reason } => {
            assert_eq!(argument, "device_id");
            assert!(reason.contains("Device not found"));
            assert!(reason.contains("invalid-device"));
        }
        _ => panic!("Expected InvalidArgument error"),
    }

    // Test DeviceEnumerationFailed conversion
    let audio_error = AudioEngineError::DeviceEnumerationFailed {
        message: "Permission denied".to_string(),
    };
    let command_error: CommandError = audio_error.into();
    match command_error {
        CommandError::ServiceError { message } => {
            assert!(message.contains("Device enumeration failed"));
            assert!(message.contains("Permission denied"));
        }
        _ => panic!("Expected ServiceError"),
    }

    // Test StreamStartFailed conversion
    let audio_error = AudioEngineError::StreamStartFailed {
        message: "Device busy".to_string(),
    };
    let command_error: CommandError = audio_error.into();
    match command_error {
        CommandError::ServiceError { message } => {
            assert!(message.contains("Stream start failed"));
        }
        _ => panic!("Expected ServiceError"),
    }

    // Test StreamStopFailed conversion
    let audio_error = AudioEngineError::StreamStopFailed {
        message: "Stream not found".to_string(),
    };
    let command_error: CommandError = audio_error.into();
    match command_error {
        CommandError::ServiceError { message } => {
            assert!(message.contains("Stream stop failed"));
        }
        _ => panic!("Expected ServiceError"),
    }

    // Test DeviceSwitchFailed conversion
    let audio_error = AudioEngineError::DeviceSwitchFailed {
        message: "Device unavailable".to_string(),
    };
    let command_error: CommandError = audio_error.into();
    match command_error {
        CommandError::ServiceError { message } => {
            assert!(message.contains("Device switch failed"));
        }
        _ => panic!("Expected ServiceError"),
    }

    // Test InvalidConfiguration conversion
    let audio_error = AudioEngineError::InvalidConfiguration {
        message: "Invalid sample rate".to_string(),
    };
    let command_error: CommandError = audio_error.into();
    match command_error {
        CommandError::ServiceError { message } => {
            assert!(message.contains("Invalid configuration"));
        }
        _ => panic!("Expected ServiceError"),
    }
}

#[test]
fn test_audio_device_serialization() {
    // Test that AudioDevice can be serialized (required for Tauri IPC)
    let device = AudioDevice::new("device-1".to_string(), "Test Device".to_string(), true);

    // Serialize to JSON
    let json = serde_json::to_string(&device).expect("Should serialize AudioDevice");
    assert!(json.contains("device-1"));
    assert!(json.contains("Test Device"));
    assert!(json.contains("true")); // is_input

    // Deserialize from JSON
    let deserialized: AudioDevice =
        serde_json::from_str(&json).expect("Should deserialize AudioDevice");
    assert_eq!(deserialized.id, "device-1");
    assert_eq!(deserialized.name, "Test Device");
    assert!(deserialized.is_input);
}

#[tokio::test]
async fn test_audio_service_error_propagation() {
    // Create a mock audio engine that returns errors
    struct ErrorAudioEngine;

    #[async_trait::async_trait]
    impl AudioEngine for ErrorAudioEngine {
        async fn enumerate_input_devices(&self) -> Result<Vec<AudioDevice>, AudioEngineError> {
            Err(AudioEngineError::DeviceEnumerationFailed {
                message: "Test error".to_string(),
            })
        }

        async fn enumerate_output_devices(&self) -> Result<Vec<AudioDevice>, AudioEngineError> {
            Err(AudioEngineError::DeviceEnumerationFailed {
                message: "Test error".to_string(),
            })
        }

        async fn get_input_device(&self) -> Result<Option<AudioDevice>, AudioEngineError> {
            Err(AudioEngineError::DeviceEnumerationFailed {
                message: "Test error".to_string(),
            })
        }

        async fn get_output_device(&self) -> Result<Option<AudioDevice>, AudioEngineError> {
            Err(AudioEngineError::DeviceEnumerationFailed {
                message: "Test error".to_string(),
            })
        }

        async fn set_input_device(&self, _device_id: &str) -> Result<(), AudioEngineError> {
            Err(AudioEngineError::DeviceNotFound {
                device_id: "test".to_string(),
            })
        }

        async fn set_output_device(&self, _device_id: &str) -> Result<(), AudioEngineError> {
            Err(AudioEngineError::DeviceNotFound {
                device_id: "test".to_string(),
            })
        }

        async fn start_input_stream(&self) -> Result<String, AudioEngineError> {
            Err(AudioEngineError::StreamStartFailed {
                message: "Test error".to_string(),
            })
        }

        async fn start_output_stream(&self) -> Result<String, AudioEngineError> {
            Err(AudioEngineError::StreamStartFailed {
                message: "Test error".to_string(),
            })
        }

        async fn stop_stream(&self, _handle: &String) -> Result<(), AudioEngineError> {
            Err(AudioEngineError::StreamStopFailed {
                message: "Test error".to_string(),
            })
        }

        async fn mute_input(&self) -> Result<(), AudioEngineError> {
            Ok(())
        }

        async fn unmute_input(&self) -> Result<(), AudioEngineError> {
            Ok(())
        }

        async fn is_input_muted(&self) -> Result<bool, AudioEngineError> {
            Ok(false)
        }

        async fn get_input_level(&self) -> Result<f32, AudioEngineError> {
            Ok(0.0)
        }

        async fn get_output_level(&self) -> Result<f32, AudioEngineError> {
            Ok(0.0)
        }
    }

    let engine: Arc<dyn AudioEngine> = Arc::new(ErrorAudioEngine);
    let service = AudioService::new(engine);

    // Test that errors are properly propagated
    let result = service.list_input_devices().await;
    assert!(result.is_err());
    assert!(matches!(
        result.unwrap_err(),
        AudioEngineError::DeviceEnumerationFailed { .. }
    ));

    let result = service.set_input_device("test").await;
    assert!(result.is_err());
    assert!(matches!(
        result.unwrap_err(),
        AudioEngineError::DeviceNotFound { .. }
    ));
}

#[test]
fn test_command_error_serialization() {
    // Test that CommandError can be serialized (required for Tauri IPC)
    let error = CommandError::InvalidArgument {
        argument: "device_id".to_string(),
        reason: "Device not found: test".to_string(),
    };

    let json = serde_json::to_string(&error).expect("Should serialize CommandError");
    assert!(json.contains("device_id"));
    assert!(json.contains("Device not found"));
}
