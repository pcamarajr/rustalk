// Audio service - Orchestrates audio device enumeration operations

use crate::domain::errors::AudioEngineError;
use crate::domain::traits::audio_engine::{AudioDevice, AudioEngine};
use std::sync::Arc;

/// Audio service managing audio device enumeration
pub struct AudioService {
    /// Audio engine for platform-specific operations (dependency injection)
    audio_engine: Arc<dyn AudioEngine>,
}

impl AudioService {
    /// Create a new AudioService with an audio engine
    ///
    /// # Arguments
    /// * `engine` - Audio engine implementation for platform-specific operations
    pub fn new(engine: Arc<dyn AudioEngine>) -> Self {
        Self {
            audio_engine: engine,
        }
    }

    /// List all available input audio devices
    ///
    /// Delegates to the audio engine's `enumerate_input_devices()` method.
    ///
    /// # Returns
    /// * `Ok(Vec<AudioDevice>)` - List of available input devices
    /// * `Err(AudioEngineError)` - Error enumerating devices
    pub async fn list_input_devices(&self) -> Result<Vec<AudioDevice>, AudioEngineError> {
        self.audio_engine.enumerate_input_devices().await
    }

    /// List all available output audio devices
    ///
    /// Delegates to the audio engine's `enumerate_output_devices()` method.
    ///
    /// # Returns
    /// * `Ok(Vec<AudioDevice>)` - List of available output devices
    /// * `Err(AudioEngineError)` - Error enumerating devices
    pub async fn list_output_devices(&self) -> Result<Vec<AudioDevice>, AudioEngineError> {
        self.audio_engine.enumerate_output_devices().await
    }

    /// Get the currently selected input device
    ///
    /// Delegates to the audio engine's `get_input_device()` method.
    ///
    /// # Returns
    /// * `Ok(Some(AudioDevice))` - Current input device
    /// * `Ok(None)` - No input device selected
    /// * `Err(AudioEngineError)` - Error retrieving device
    pub async fn get_input_device(&self) -> Result<Option<AudioDevice>, AudioEngineError> {
        self.audio_engine.get_input_device().await
    }

    /// Get the currently selected output device
    ///
    /// Delegates to the audio engine's `get_output_device()` method.
    ///
    /// # Returns
    /// * `Ok(Some(AudioDevice))` - Current output device
    /// * `Ok(None)` - No output device selected
    /// * `Err(AudioEngineError)` - Error retrieving device
    pub async fn get_output_device(&self) -> Result<Option<AudioDevice>, AudioEngineError> {
        self.audio_engine.get_output_device().await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::traits::audio_engine::AudioEngine;
    use async_trait::async_trait;
    use std::sync::Arc;
    use tokio::sync::Mutex;

    // Mock audio engine for testing
    // Uses Arc<Mutex<>> for thread-safe interior mutability since async_trait requires Send + Sync
    struct MockAudioEngine {
        input_devices: Arc<Mutex<Vec<AudioDevice>>>,
        output_devices: Arc<Mutex<Vec<AudioDevice>>>,
        current_input: Arc<Mutex<Option<String>>>,
        current_output: Arc<Mutex<Option<String>>>,
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

        async fn start_input_stream(&self) -> Result<String, AudioEngineError> {
            // Not used in enumeration tests
            Ok("stream-1".to_string())
        }

        async fn start_output_stream(&self) -> Result<String, AudioEngineError> {
            // Not used in enumeration tests
            Ok("stream-1".to_string())
        }

        async fn stop_stream(&self, _handle: &String) -> Result<(), AudioEngineError> {
            // Not used in enumeration tests
            Ok(())
        }

        async fn mute_input(&self) -> Result<(), AudioEngineError> {
            // Not used in enumeration tests
            Ok(())
        }

        async fn unmute_input(&self) -> Result<(), AudioEngineError> {
            // Not used in enumeration tests
            Ok(())
        }

        async fn is_input_muted(&self) -> Result<bool, AudioEngineError> {
            // Not used in enumeration tests
            Ok(false)
        }

        async fn get_input_level(&self) -> Result<f32, AudioEngineError> {
            // Not used in enumeration tests
            Ok(0.0)
        }

        async fn get_output_level(&self) -> Result<f32, AudioEngineError> {
            // Not used in enumeration tests
            Ok(0.0)
        }
    }

    #[tokio::test]
    async fn test_new_audio_service() {
        let mock_engine: Arc<dyn AudioEngine> = Arc::new(MockAudioEngine::new());
        let _service = AudioService::new(mock_engine);
        // Service should be created successfully - if we get here without panicking, it works
    }

    #[tokio::test]
    async fn test_list_input_devices() {
        let mock_engine: Arc<dyn AudioEngine> = Arc::new(MockAudioEngine::new());
        let service = AudioService::new(mock_engine);
        let devices = service.list_input_devices().await.unwrap();

        assert_eq!(devices.len(), 2);
        assert!(devices.iter().all(|d| d.is_input));
        assert_eq!(devices[0].id, "input-1");
        assert_eq!(devices[0].name, "Built-in Microphone");
        assert_eq!(devices[1].id, "input-2");
        assert_eq!(devices[1].name, "USB Microphone");
    }

    #[tokio::test]
    async fn test_list_output_devices() {
        let mock_engine: Arc<dyn AudioEngine> = Arc::new(MockAudioEngine::new());
        let service = AudioService::new(mock_engine);
        let devices = service.list_output_devices().await.unwrap();

        assert_eq!(devices.len(), 2);
        assert!(devices.iter().all(|d| !d.is_input));
        assert_eq!(devices[0].id, "output-1");
        assert_eq!(devices[0].name, "Built-in Speakers");
        assert_eq!(devices[1].id, "output-2");
        assert_eq!(devices[1].name, "USB Headphones");
    }

    #[tokio::test]
    async fn test_get_input_device() {
        let mock_engine: Arc<dyn AudioEngine> = Arc::new(MockAudioEngine::new());
        let service = AudioService::new(Arc::clone(&mock_engine));

        // Initially no device selected
        assert!(service.get_input_device().await.unwrap().is_none());

        // Set input device using the trait method
        mock_engine.set_input_device("input-1").await.unwrap();
        let device = service.get_input_device().await.unwrap().unwrap();
        assert_eq!(device.id, "input-1");
        assert_eq!(device.name, "Built-in Microphone");
    }

    #[tokio::test]
    async fn test_get_output_device() {
        let mock_engine: Arc<dyn AudioEngine> = Arc::new(MockAudioEngine::new());
        let service = AudioService::new(Arc::clone(&mock_engine));

        // Initially no device selected
        assert!(service.get_output_device().await.unwrap().is_none());

        // Set output device using the trait method
        mock_engine.set_output_device("output-1").await.unwrap();
        let device = service.get_output_device().await.unwrap().unwrap();
        assert_eq!(device.id, "output-1");
        assert_eq!(device.name, "Built-in Speakers");
    }

    #[tokio::test]
    async fn test_get_input_device_none() {
        let mock_engine: Arc<dyn AudioEngine> = Arc::new(MockAudioEngine::new());
        let service = AudioService::new(mock_engine);

        // No device selected
        let device = service.get_input_device().await.unwrap();
        assert!(device.is_none());
    }

    #[tokio::test]
    async fn test_get_output_device_none() {
        let mock_engine: Arc<dyn AudioEngine> = Arc::new(MockAudioEngine::new());
        let service = AudioService::new(mock_engine);

        // No device selected
        let device = service.get_output_device().await.unwrap();
        assert!(device.is_none());
    }
}
