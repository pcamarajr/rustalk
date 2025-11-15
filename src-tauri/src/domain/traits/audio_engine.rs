// AudioEngine trait - Abstraction for platform-specific audio operations
// Platform-specific implementations will be in infrastructure layer

use crate::domain::errors::AudioEngineError;
use async_trait::async_trait;

/// Audio device information
///
/// Represents an audio input or output device with its identifier and display name.
/// This is a minimal representation for AUD-5.1; full entity can be refined in AUD-5.3.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AudioDevice {
    /// Unique identifier for the device (platform-agnostic)
    pub id: String,
    /// Human-readable device name
    pub name: String,
    /// Whether this is an input device (microphone) or output device (speaker)
    pub is_input: bool,
}

impl AudioDevice {
    /// Create a new AudioDevice
    pub fn new(id: String, name: String, is_input: bool) -> Self {
        Self { id, name, is_input }
    }
}

/// Stream handle identifier
///
/// Used to identify and manage multiple concurrent audio streams.
pub type StreamHandle = String;

/// Audio level measurement (0.0 to 1.0)
///
/// Represents the current audio level for visualization purposes.
pub type AudioLevel = f32;

/// Trait for platform-specific audio operations
///
/// This trait abstracts platform-specific audio implementations (e.g., cpal-based backends).
/// All operations are async to support non-blocking I/O and Tokio integration.
/// The trait must be `Send + Sync` for use in `Arc<dyn AudioEngine>`.
#[async_trait]
pub trait AudioEngine: Send + Sync {
    /// Enumerate all available input audio devices
    ///
    /// # Returns
    ///
    /// * `Ok(Vec<AudioDevice>)` - List of available input devices
    /// * `Err(AudioEngineError::DeviceEnumerationFailed)` - Failed to enumerate devices
    ///
    /// # Errors
    ///
    /// * `AudioEngineError::DeviceEnumerationFailed` - Platform enumeration error
    async fn enumerate_input_devices(&self) -> Result<Vec<AudioDevice>, AudioEngineError>;

    /// Enumerate all available output audio devices
    ///
    /// # Returns
    ///
    /// * `Ok(Vec<AudioDevice>)` - List of available output devices
    /// * `Err(AudioEngineError::DeviceEnumerationFailed)` - Failed to enumerate devices
    ///
    /// # Errors
    ///
    /// * `AudioEngineError::DeviceEnumerationFailed` - Platform enumeration error
    async fn enumerate_output_devices(&self) -> Result<Vec<AudioDevice>, AudioEngineError>;

    /// Get the currently selected input device
    ///
    /// # Returns
    ///
    /// * `Ok(Some(AudioDevice))` - Current input device
    /// * `Ok(None)` - No input device selected
    /// * `Err(AudioEngineError)` - Error retrieving device
    async fn get_input_device(&self) -> Result<Option<AudioDevice>, AudioEngineError>;

    /// Get the currently selected output device
    ///
    /// # Returns
    ///
    /// * `Ok(Some(AudioDevice))` - Current output device
    /// * `Ok(None)` - No output device selected
    /// * `Err(AudioEngineError)` - Error retrieving device
    async fn get_output_device(&self) -> Result<Option<AudioDevice>, AudioEngineError>;

    /// Set the active input device
    ///
    /// # Arguments
    ///
    /// * `device_id` - Unique identifier of the device to select
    ///
    /// # Returns
    ///
    /// * `Ok(())` - Device selected successfully
    /// * `Err(AudioEngineError::DeviceNotFound)` - Device ID doesn't exist
    /// * `Err(AudioEngineError::DeviceSwitchFailed)` - Failed to switch device
    ///
    /// # Errors
    ///
    /// * `AudioEngineError::DeviceNotFound` - Device ID not found
    /// * `AudioEngineError::DeviceSwitchFailed` - Platform switch error
    async fn set_input_device(&self, device_id: &str) -> Result<(), AudioEngineError>;

    /// Set the active output device
    ///
    /// # Arguments
    ///
    /// * `device_id` - Unique identifier of the device to select
    ///
    /// # Returns
    ///
    /// * `Ok(())` - Device selected successfully
    /// * `Err(AudioEngineError::DeviceNotFound)` - Device ID doesn't exist
    /// * `Err(AudioEngineError::DeviceSwitchFailed)` - Failed to switch device
    ///
    /// # Errors
    ///
    /// * `AudioEngineError::DeviceNotFound` - Device ID not found
    /// * `AudioEngineError::DeviceSwitchFailed` - Platform switch error
    async fn set_output_device(&self, device_id: &str) -> Result<(), AudioEngineError>;

    /// Start an audio input stream
    ///
    /// Begins capturing audio from the selected input device. Returns a stream handle
    /// that can be used to stop the stream later.
    ///
    /// # Returns
    ///
    /// * `Ok(StreamHandle)` - Stream handle for managing this stream
    /// * `Err(AudioEngineError::StreamStartFailed)` - Failed to start stream
    /// * `Err(AudioEngineError::InvalidConfiguration)` - Invalid audio configuration
    ///
    /// # Errors
    ///
    /// * `AudioEngineError::StreamStartFailed` - Platform stream error
    /// * `AudioEngineError::InvalidConfiguration` - Invalid audio settings
    async fn start_input_stream(&self) -> Result<StreamHandle, AudioEngineError>;

    /// Start an audio output stream
    ///
    /// Begins playing audio to the selected output device. Returns a stream handle
    /// that can be used to stop the stream later.
    ///
    /// # Returns
    ///
    /// * `Ok(StreamHandle)` - Stream handle for managing this stream
    /// * `Err(AudioEngineError::StreamStartFailed)` - Failed to start stream
    /// * `Err(AudioEngineError::InvalidConfiguration)` - Invalid audio configuration
    ///
    /// # Errors
    ///
    /// * `AudioEngineError::StreamStartFailed` - Platform stream error
    /// * `AudioEngineError::InvalidConfiguration` - Invalid audio settings
    async fn start_output_stream(&self) -> Result<StreamHandle, AudioEngineError>;

    /// Stop an audio stream
    ///
    /// # Arguments
    ///
    /// * `handle` - Stream handle returned from `start_input_stream` or `start_output_stream`
    ///
    /// # Returns
    ///
    /// * `Ok(())` - Stream stopped successfully
    /// * `Err(AudioEngineError::StreamStopFailed)` - Failed to stop stream
    ///
    /// # Errors
    ///
    /// * `AudioEngineError::StreamStopFailed` - Platform stream error
    async fn stop_stream(&self, handle: &StreamHandle) -> Result<(), AudioEngineError>;

    /// Mute the audio input
    ///
    /// # Returns
    ///
    /// * `Ok(())` - Input muted successfully
    /// * `Err(AudioEngineError)` - Failed to mute
    async fn mute_input(&self) -> Result<(), AudioEngineError>;

    /// Unmute the audio input
    ///
    /// # Returns
    ///
    /// * `Ok(())` - Input unmuted successfully
    /// * `Err(AudioEngineError)` - Failed to unmute
    async fn unmute_input(&self) -> Result<(), AudioEngineError>;

    /// Check if input is currently muted
    ///
    /// # Returns
    ///
    /// * `Ok(bool)` - `true` if muted, `false` if unmuted
    /// * `Err(AudioEngineError)` - Error checking mute status
    async fn is_input_muted(&self) -> Result<bool, AudioEngineError>;

    /// Get current input audio level
    ///
    /// Returns the current audio level (0.0 to 1.0) for UI visualization.
    ///
    /// # Returns
    ///
    /// * `Ok(AudioLevel)` - Current audio level (0.0 = silent, 1.0 = maximum)
    /// * `Err(AudioEngineError)` - Error reading audio level
    async fn get_input_level(&self) -> Result<AudioLevel, AudioEngineError>;

    /// Get current output audio level
    ///
    /// Returns the current audio level (0.0 to 1.0) for UI visualization.
    ///
    /// # Returns
    ///
    /// * `Ok(AudioLevel)` - Current audio level (0.0 = silent, 1.0 = maximum)
    /// * `Err(AudioEngineError)` - Error reading audio level
    async fn get_output_level(&self) -> Result<AudioLevel, AudioEngineError>;
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use tokio::sync::Mutex;

    // Mock implementation for testing trait contract
    // Uses Arc<Mutex<>> for thread-safe interior mutability since async_trait requires Send + Sync
    struct MockAudioEngine {
        input_devices: Arc<Mutex<Vec<AudioDevice>>>,
        output_devices: Arc<Mutex<Vec<AudioDevice>>>,
        current_input: Arc<Mutex<Option<String>>>,
        current_output: Arc<Mutex<Option<String>>>,
        streams: Arc<Mutex<std::collections::HashMap<String, bool>>>,
        input_muted: Arc<Mutex<bool>>,
        input_level: Arc<Mutex<AudioLevel>>,
        output_level: Arc<Mutex<AudioLevel>>,
        next_stream_id: Arc<Mutex<u32>>,
    }

    impl MockAudioEngine {
        fn new() -> Self {
            // Initialize with some default mock devices
            let input_devices = vec![
                AudioDevice::new(
                    "input-1".to_string(),
                    "Built-in Microphone".to_string(),
                    true,
                ),
                AudioDevice::new("input-2".to_string(), "USB Microphone".to_string(), true),
            ];
            let output_devices = vec![
                AudioDevice::new(
                    "output-1".to_string(),
                    "Built-in Speakers".to_string(),
                    false,
                ),
                AudioDevice::new("output-2".to_string(), "USB Headphones".to_string(), false),
            ];

            Self {
                input_devices: Arc::new(Mutex::new(input_devices)),
                output_devices: Arc::new(Mutex::new(output_devices)),
                current_input: Arc::new(Mutex::new(None)),
                current_output: Arc::new(Mutex::new(None)),
                streams: Arc::new(Mutex::new(std::collections::HashMap::new())),
                input_muted: Arc::new(Mutex::new(false)),
                input_level: Arc::new(Mutex::new(0.0)),
                output_level: Arc::new(Mutex::new(0.0)),
                next_stream_id: Arc::new(Mutex::new(1)),
            }
        }
    }

    #[async_trait]
    impl AudioEngine for MockAudioEngine {
        async fn enumerate_input_devices(&self) -> Result<Vec<AudioDevice>, AudioEngineError> {
            let devices = self.input_devices.lock().await;
            Ok(devices.clone())
        }

        async fn enumerate_output_devices(&self) -> Result<Vec<AudioDevice>, AudioEngineError> {
            let devices = self.output_devices.lock().await;
            Ok(devices.clone())
        }

        async fn get_input_device(&self) -> Result<Option<AudioDevice>, AudioEngineError> {
            let current_id = self.current_input.lock().await.clone();
            if let Some(id) = current_id {
                let devices = self.input_devices.lock().await;
                Ok(devices.iter().find(|d| d.id == id).cloned())
            } else {
                Ok(None)
            }
        }

        async fn get_output_device(&self) -> Result<Option<AudioDevice>, AudioEngineError> {
            let current_id = self.current_output.lock().await.clone();
            if let Some(id) = current_id {
                let devices = self.output_devices.lock().await;
                Ok(devices.iter().find(|d| d.id == id).cloned())
            } else {
                Ok(None)
            }
        }

        async fn set_input_device(&self, device_id: &str) -> Result<(), AudioEngineError> {
            let devices = self.input_devices.lock().await;
            if !devices.iter().any(|d| d.id == device_id) {
                return Err(AudioEngineError::DeviceNotFound {
                    device_id: device_id.to_string(),
                });
            }
            drop(devices);
            let mut current = self.current_input.lock().await;
            *current = Some(device_id.to_string());
            Ok(())
        }

        async fn set_output_device(&self, device_id: &str) -> Result<(), AudioEngineError> {
            let devices = self.output_devices.lock().await;
            if !devices.iter().any(|d| d.id == device_id) {
                return Err(AudioEngineError::DeviceNotFound {
                    device_id: device_id.to_string(),
                });
            }
            drop(devices);
            let mut current = self.current_output.lock().await;
            *current = Some(device_id.to_string());
            Ok(())
        }

        async fn start_input_stream(&self) -> Result<StreamHandle, AudioEngineError> {
            let mut id = self.next_stream_id.lock().await;
            let handle = format!("input-stream-{}", *id);
            *id += 1;
            drop(id);

            let mut streams = self.streams.lock().await;
            streams.insert(handle.clone(), true);
            Ok(handle)
        }

        async fn start_output_stream(&self) -> Result<StreamHandle, AudioEngineError> {
            let mut id = self.next_stream_id.lock().await;
            let handle = format!("output-stream-{}", *id);
            *id += 1;
            drop(id);

            let mut streams = self.streams.lock().await;
            streams.insert(handle.clone(), true);
            Ok(handle)
        }

        async fn stop_stream(&self, handle: &StreamHandle) -> Result<(), AudioEngineError> {
            let mut streams = self.streams.lock().await;
            if streams.remove(handle).is_none() {
                return Err(AudioEngineError::StreamStopFailed {
                    message: format!("Stream not found: {}", handle),
                });
            }
            Ok(())
        }

        async fn mute_input(&self) -> Result<(), AudioEngineError> {
            let mut muted = self.input_muted.lock().await;
            *muted = true;
            Ok(())
        }

        async fn unmute_input(&self) -> Result<(), AudioEngineError> {
            let mut muted = self.input_muted.lock().await;
            *muted = false;
            Ok(())
        }

        async fn is_input_muted(&self) -> Result<bool, AudioEngineError> {
            let muted = self.input_muted.lock().await;
            Ok(*muted)
        }

        async fn get_input_level(&self) -> Result<AudioLevel, AudioEngineError> {
            let level = self.input_level.lock().await;
            Ok(*level)
        }

        async fn get_output_level(&self) -> Result<AudioLevel, AudioEngineError> {
            let level = self.output_level.lock().await;
            Ok(*level)
        }
    }

    #[tokio::test]
    async fn test_enumerate_input_devices() {
        let engine = MockAudioEngine::new();
        let devices = engine.enumerate_input_devices().await.unwrap();

        assert_eq!(devices.len(), 2);
        assert!(devices.iter().all(|d| d.is_input));
        assert_eq!(devices[0].id, "input-1");
        assert_eq!(devices[0].name, "Built-in Microphone");
    }

    #[tokio::test]
    async fn test_enumerate_output_devices() {
        let engine = MockAudioEngine::new();
        let devices = engine.enumerate_output_devices().await.unwrap();

        assert_eq!(devices.len(), 2);
        assert!(devices.iter().all(|d| !d.is_input));
        assert_eq!(devices[0].id, "output-1");
        assert_eq!(devices[0].name, "Built-in Speakers");
    }

    #[tokio::test]
    async fn test_set_and_get_input_device() {
        let engine = MockAudioEngine::new();

        // Initially no device selected
        assert!(engine.get_input_device().await.unwrap().is_none());

        // Set input device
        engine.set_input_device("input-1").await.unwrap();
        let device = engine.get_input_device().await.unwrap().unwrap();
        assert_eq!(device.id, "input-1");
        assert_eq!(device.name, "Built-in Microphone");
    }

    #[tokio::test]
    async fn test_set_and_get_output_device() {
        let engine = MockAudioEngine::new();

        // Initially no device selected
        assert!(engine.get_output_device().await.unwrap().is_none());

        // Set output device
        engine.set_output_device("output-1").await.unwrap();
        let device = engine.get_output_device().await.unwrap().unwrap();
        assert_eq!(device.id, "output-1");
        assert_eq!(device.name, "Built-in Speakers");
    }

    #[tokio::test]
    async fn test_set_invalid_device() {
        let engine = MockAudioEngine::new();

        let result = engine.set_input_device("invalid-device").await;
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            AudioEngineError::DeviceNotFound { .. }
        ));
    }

    #[tokio::test]
    async fn test_start_and_stop_input_stream() {
        let engine = MockAudioEngine::new();

        let handle = engine.start_input_stream().await.unwrap();
        assert!(handle.starts_with("input-stream-"));

        engine.stop_stream(&handle).await.unwrap();

        // Stopping again should fail
        let result = engine.stop_stream(&handle).await;
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            AudioEngineError::StreamStopFailed { .. }
        ));
    }

    #[tokio::test]
    async fn test_start_and_stop_output_stream() {
        let engine = MockAudioEngine::new();

        let handle = engine.start_output_stream().await.unwrap();
        assert!(handle.starts_with("output-stream-"));

        engine.stop_stream(&handle).await.unwrap();
    }

    #[tokio::test]
    async fn test_multiple_streams() {
        let engine = MockAudioEngine::new();

        let handle1 = engine.start_input_stream().await.unwrap();
        let handle2 = engine.start_output_stream().await.unwrap();
        let handle3 = engine.start_input_stream().await.unwrap();

        assert_ne!(handle1, handle2);
        assert_ne!(handle1, handle3);
        assert_ne!(handle2, handle3);

        engine.stop_stream(&handle1).await.unwrap();
        engine.stop_stream(&handle2).await.unwrap();
        engine.stop_stream(&handle3).await.unwrap();
    }

    #[tokio::test]
    async fn test_mute_and_unmute() {
        let engine = MockAudioEngine::new();

        // Initially not muted
        assert!(!engine.is_input_muted().await.unwrap());

        // Mute
        engine.mute_input().await.unwrap();
        assert!(engine.is_input_muted().await.unwrap());

        // Unmute
        engine.unmute_input().await.unwrap();
        assert!(!engine.is_input_muted().await.unwrap());
    }

    #[tokio::test]
    async fn test_audio_levels() {
        let engine = MockAudioEngine::new();

        let input_level = engine.get_input_level().await.unwrap();
        let output_level = engine.get_output_level().await.unwrap();

        assert_eq!(input_level, 0.0);
        assert_eq!(output_level, 0.0);
        assert!(input_level >= 0.0 && input_level <= 1.0);
        assert!(output_level >= 0.0 && output_level <= 1.0);
    }

    #[tokio::test]
    async fn test_device_switching() {
        let engine = MockAudioEngine::new();

        // Set initial device
        engine.set_input_device("input-1").await.unwrap();
        assert_eq!(
            engine.get_input_device().await.unwrap().unwrap().id,
            "input-1"
        );

        // Switch to different device
        engine.set_input_device("input-2").await.unwrap();
        assert_eq!(
            engine.get_input_device().await.unwrap().unwrap().id,
            "input-2"
        );
    }
}
