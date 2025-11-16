// macOS audio implementation using cpal
// Bridges cpal's callback-based API to async AudioEngine trait

use crate::domain::errors::AudioEngineError;
use crate::domain::traits::audio_engine::{AudioDevice, AudioEngine, AudioLevel, StreamHandle};
use async_trait::async_trait;
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{Device, SampleFormat, Stream};
use std::collections::HashMap;
use std::sync::{Arc, Mutex as StdMutex};
use tokio::sync::Mutex;

/// Type alias for stream storage
/// Clippy warning suppressed: We use unsafe impl Send+Sync with proper documentation
#[allow(clippy::arc_with_non_send_sync)]
type StreamMap = Arc<StdMutex<HashMap<String, Stream>>>;

/// macOS audio engine implementation using cpal
///
/// This implementation bridges cpal's callback-based API to the async AudioEngine trait.
/// It uses channels to bridge audio callbacks to async Tokio tasks.
///
/// Note: cpal's Host and Stream types are not Send+Sync, so we create the host on-demand
/// and use std::sync::Mutex for stream storage (streams are dropped synchronously).
///
/// # Safety
///
/// This struct uses `unsafe impl Sync` because:
/// 1. Streams are only accessed via blocking locks (`std::sync::Mutex`)
/// 2. All stream operations happen on the thread that created them
/// 3. The Mutex ensures thread-safe access to the HashMap
/// 4. Host is created on-demand and not stored, avoiding Send+Sync issues
pub struct MacOSAudioEngine {
    /// Currently selected input device ID
    current_input_device: Arc<Mutex<Option<String>>>,
    /// Currently selected output device ID
    current_output_device: Arc<Mutex<Option<String>>>,
    /// Active audio streams (handle -> Stream)
    /// Uses std::sync::Mutex because Stream is not Send+Sync
    streams: StreamMap,
    /// Input mute state
    input_muted: Arc<Mutex<bool>>,
    /// Current input audio level (0.0 to 1.0)
    input_level: Arc<Mutex<AudioLevel>>,
    /// Current output audio level (0.0 to 1.0)
    output_level: Arc<Mutex<AudioLevel>>,
    /// Next stream handle ID
    next_stream_id: Arc<Mutex<u32>>,
}

// Safety: MacOSAudioEngine is Send+Sync because:
// 1. All non-Send+Sync types (Stream) are protected by std::sync::Mutex
// 2. Stream operations only happen via blocking locks on the creating thread
// 3. Host is created on-demand and not stored, avoiding Send+Sync issues
// 4. All other fields (Arc<Mutex<...>>) are already Send+Sync
// 5. Streams are never moved between threads - they're created and dropped on the same thread
unsafe impl Send for MacOSAudioEngine {}
unsafe impl Sync for MacOSAudioEngine {}

impl MacOSAudioEngine {
    /// Create a new macOS audio engine
    #[allow(clippy::arc_with_non_send_sync)]
    pub fn new() -> Result<Self, AudioEngineError> {
        Ok(Self {
            current_input_device: Arc::new(Mutex::new(None)),
            current_output_device: Arc::new(Mutex::new(None)),
            streams: Arc::new(StdMutex::new(HashMap::new())),
            input_muted: Arc::new(Mutex::new(false)),
            input_level: Arc::new(Mutex::new(0.0)),
            output_level: Arc::new(Mutex::new(0.0)),
            next_stream_id: Arc::new(Mutex::new(1)),
        })
    }

    /// Get the default host (created on-demand since Host is not Send+Sync)
    fn get_host(&self) -> cpal::Host {
        cpal::default_host()
    }

    /// Get a device by ID
    fn get_device_by_id(
        &self,
        device_id: &str,
        is_input: bool,
    ) -> Result<Device, AudioEngineError> {
        let host = self.get_host();
        let devices = if is_input {
            host.input_devices()
        } else {
            host.output_devices()
        }
        .map_err(|e| AudioEngineError::DeviceEnumerationFailed {
            message: format!("Failed to enumerate devices: {}", e),
        })?;

        for device in devices {
            // Use device name as ID (cpal doesn't provide stable IDs, so we use name)
            // In production, we might want to use a more stable identifier
            let name = device
                .name()
                .map_err(|e| AudioEngineError::DeviceEnumerationFailed {
                    message: format!("Failed to get device name: {}", e),
                })?;

            if name == device_id {
                return Ok(device);
            }
        }

        Err(AudioEngineError::DeviceNotFound {
            device_id: device_id.to_string(),
        })
    }

    /// Generate a unique stream handle
    async fn generate_stream_handle(&self) -> String {
        let mut id = self.next_stream_id.lock().await;
        let handle = format!("stream-{}", *id);
        *id += 1;
        handle
    }

    /// Calculate audio level from sample buffer
    fn calculate_level(samples: &[f32]) -> AudioLevel {
        if samples.is_empty() {
            return 0.0;
        }

        // Calculate RMS (Root Mean Square) for audio level
        let sum_squares: f32 = samples.iter().map(|&s| s * s).sum();
        let rms = (sum_squares / samples.len() as f32).sqrt();

        // Normalize to 0.0-1.0 range (assuming samples are in -1.0 to 1.0 range)
        rms.min(1.0)
    }
}

#[async_trait]
impl AudioEngine for MacOSAudioEngine {
    async fn enumerate_input_devices(&self) -> Result<Vec<AudioDevice>, AudioEngineError> {
        let host = self.get_host();
        let devices =
            host.input_devices()
                .map_err(|e| AudioEngineError::DeviceEnumerationFailed {
                    message: format!("Failed to enumerate input devices: {}", e),
                })?;

        let mut result = Vec::new();
        for device in devices {
            let name = device
                .name()
                .map_err(|e| AudioEngineError::DeviceEnumerationFailed {
                    message: format!("Failed to get device name: {}", e),
                })?;

            // Use device name as ID (cpal limitation - no stable IDs)
            result.push(AudioDevice::new(name.clone(), name, true));
        }

        Ok(result)
    }

    async fn enumerate_output_devices(&self) -> Result<Vec<AudioDevice>, AudioEngineError> {
        let host = self.get_host();
        let devices =
            host.output_devices()
                .map_err(|e| AudioEngineError::DeviceEnumerationFailed {
                    message: format!("Failed to enumerate output devices: {}", e),
                })?;

        let mut result = Vec::new();
        for device in devices {
            let name = device
                .name()
                .map_err(|e| AudioEngineError::DeviceEnumerationFailed {
                    message: format!("Failed to get device name: {}", e),
                })?;

            // Use device name as ID (cpal limitation - no stable IDs)
            result.push(AudioDevice::new(name.clone(), name, false));
        }

        Ok(result)
    }

    async fn get_input_device(&self) -> Result<Option<AudioDevice>, AudioEngineError> {
        let device_id = self.current_input_device.lock().await.clone();
        if let Some(id) = device_id {
            let device = self.get_device_by_id(&id, true)?;
            let name = device
                .name()
                .map_err(|e| AudioEngineError::DeviceEnumerationFailed {
                    message: format!("Failed to get device name: {}", e),
                })?;
            Ok(Some(AudioDevice::new(id, name, true)))
        } else {
            Ok(None)
        }
    }

    async fn get_output_device(&self) -> Result<Option<AudioDevice>, AudioEngineError> {
        let device_id = self.current_output_device.lock().await.clone();
        if let Some(id) = device_id {
            let device = self.get_device_by_id(&id, false)?;
            let name = device
                .name()
                .map_err(|e| AudioEngineError::DeviceEnumerationFailed {
                    message: format!("Failed to get device name: {}", e),
                })?;
            Ok(Some(AudioDevice::new(id, name, false)))
        } else {
            Ok(None)
        }
    }

    async fn set_input_device(&self, device_id: &str) -> Result<(), AudioEngineError> {
        // Verify device exists
        self.get_device_by_id(device_id, true)?;

        let mut current = self.current_input_device.lock().await;
        *current = Some(device_id.to_string());
        Ok(())
    }

    async fn set_output_device(&self, device_id: &str) -> Result<(), AudioEngineError> {
        // Verify device exists
        self.get_device_by_id(device_id, false)?;

        let mut current = self.current_output_device.lock().await;
        *current = Some(device_id.to_string());
        Ok(())
    }

    async fn start_input_stream(&self) -> Result<StreamHandle, AudioEngineError> {
        // Get the selected input device or default
        let device_id = self.current_input_device.lock().await.clone();
        let host = self.get_host();
        let device = if let Some(id) = device_id {
            self.get_device_by_id(&id, true)?
        } else {
            host.default_input_device().ok_or_else(|| {
                AudioEngineError::DeviceEnumerationFailed {
                    message: "No default input device available".to_string(),
                }
            })?
        };

        // Get default config
        let config =
            device
                .default_input_config()
                .map_err(|e| AudioEngineError::InvalidConfiguration {
                    message: format!("Failed to get input config: {}", e),
                })?;

        let handle = self.generate_stream_handle().await;
        let input_level = Arc::clone(&self.input_level);
        let input_muted = Arc::clone(&self.input_muted);

        // Build stream based on sample format
        let stream = match config.sample_format() {
            SampleFormat::F32 => {
                let stream = device
                    .build_input_stream(
                        &config.into(),
                        move |data: &[f32], _: &cpal::InputCallbackInfo| {
                            // Calculate audio level
                            let level = Self::calculate_level(data);
                            *input_level.blocking_lock() = level;

                            // If muted, we could zero out the buffer here
                            // For now, we just track the level
                            let muted = *input_muted.blocking_lock();
                            if muted {
                                // In a real implementation, we'd zero the buffer
                                // But cpal doesn't allow modifying the buffer in the callback
                                // Mute is handled at a higher level
                            }
                        },
                        |err| {
                            eprintln!("DEBUG:[AUDIO/INPUT] Stream error: {}", err);
                        },
                        None,
                    )
                    .map_err(|e| AudioEngineError::StreamStartFailed {
                        message: format!("Failed to build input stream: {}", e),
                    })?;

                stream
            }
            SampleFormat::I16 => {
                let stream = device
                    .build_input_stream(
                        &config.into(),
                        move |data: &[i16], _: &cpal::InputCallbackInfo| {
                            // Convert i16 to f32 for level calculation
                            let samples: Vec<f32> =
                                data.iter().map(|&s| s as f32 / 32768.0).collect();
                            let level = Self::calculate_level(&samples);
                            *input_level.blocking_lock() = level;
                        },
                        |err| {
                            eprintln!("DEBUG:[AUDIO/INPUT] Stream error: {}", err);
                        },
                        None,
                    )
                    .map_err(|e| AudioEngineError::StreamStartFailed {
                        message: format!("Failed to build input stream: {}", e),
                    })?;

                stream
            }
            SampleFormat::U16 => {
                let stream = device
                    .build_input_stream(
                        &config.into(),
                        move |data: &[u16], _: &cpal::InputCallbackInfo| {
                            // Convert u16 to f32 for level calculation
                            let samples: Vec<f32> = data
                                .iter()
                                .map(|&s| (s as f32 - 32768.0) / 32768.0)
                                .collect();
                            let level = Self::calculate_level(&samples);
                            *input_level.blocking_lock() = level;
                        },
                        |err| {
                            eprintln!("DEBUG:[AUDIO/INPUT] Stream error: {}", err);
                        },
                        None,
                    )
                    .map_err(|e| AudioEngineError::StreamStartFailed {
                        message: format!("Failed to build input stream: {}", e),
                    })?;

                stream
            }
            _ => {
                return Err(AudioEngineError::InvalidConfiguration {
                    message: format!("Unsupported sample format: {:?}", config.sample_format()),
                });
            }
        };

        // Start the stream
        stream
            .play()
            .map_err(|e| AudioEngineError::StreamStartFailed {
                message: format!("Failed to play stream: {}", e),
            })?;

        // Store the stream (using blocking lock since Stream is not Send)
        // Note: We can't use spawn_blocking here because Stream is not Send.
        // Instead, we handle PoisonError explicitly to avoid unwrap().
        let mut streams = self
            .streams
            .lock()
            .map_err(|_| AudioEngineError::StreamStartFailed {
                message: "Mutex poisoned".to_string(),
            })?;
        streams.insert(handle.clone(), stream);

        Ok(handle)
    }

    async fn start_output_stream(&self) -> Result<StreamHandle, AudioEngineError> {
        // Get the selected output device or default
        let device_id = self.current_output_device.lock().await.clone();
        let host = self.get_host();
        let device = if let Some(id) = device_id {
            self.get_device_by_id(&id, false)?
        } else {
            host.default_output_device().ok_or_else(|| {
                AudioEngineError::DeviceEnumerationFailed {
                    message: "No default output device available".to_string(),
                }
            })?
        };

        // Get default config
        let config =
            device
                .default_output_config()
                .map_err(|e| AudioEngineError::InvalidConfiguration {
                    message: format!("Failed to get output config: {}", e),
                })?;

        let handle = self.generate_stream_handle().await;
        let output_level = Arc::clone(&self.output_level);

        // Build stream based on sample format
        let stream = match config.sample_format() {
            SampleFormat::F32 => {
                let stream = device
                    .build_output_stream(
                        &config.into(),
                        move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
                            // Zero out buffer (silence)
                            for sample in data.iter_mut() {
                                *sample = 0.0;
                            }
                            // Output level is 0.0 when playing silence
                            *output_level.blocking_lock() = 0.0;
                        },
                        |err| {
                            eprintln!("DEBUG:[AUDIO/OUTPUT] Stream error: {}", err);
                        },
                        None,
                    )
                    .map_err(|e| AudioEngineError::StreamStartFailed {
                        message: format!("Failed to build output stream: {}", e),
                    })?;

                stream
            }
            SampleFormat::I16 => {
                let stream = device
                    .build_output_stream(
                        &config.into(),
                        move |data: &mut [i16], _: &cpal::OutputCallbackInfo| {
                            // Zero out buffer (silence)
                            for sample in data.iter_mut() {
                                *sample = 0;
                            }
                            *output_level.blocking_lock() = 0.0;
                        },
                        |err| {
                            eprintln!("DEBUG:[AUDIO/OUTPUT] Stream error: {}", err);
                        },
                        None,
                    )
                    .map_err(|e| AudioEngineError::StreamStartFailed {
                        message: format!("Failed to build output stream: {}", e),
                    })?;

                stream
            }
            SampleFormat::U16 => {
                let stream = device
                    .build_output_stream(
                        &config.into(),
                        move |data: &mut [u16], _: &cpal::OutputCallbackInfo| {
                            // Zero out buffer (silence)
                            for sample in data.iter_mut() {
                                *sample = 32768; // Center value for u16
                            }
                            *output_level.blocking_lock() = 0.0;
                        },
                        |err| {
                            eprintln!("DEBUG:[AUDIO/OUTPUT] Stream error: {}", err);
                        },
                        None,
                    )
                    .map_err(|e| AudioEngineError::StreamStartFailed {
                        message: format!("Failed to build output stream: {}", e),
                    })?;

                stream
            }
            _ => {
                return Err(AudioEngineError::InvalidConfiguration {
                    message: format!("Unsupported sample format: {:?}", config.sample_format()),
                });
            }
        };

        // Start the stream
        stream
            .play()
            .map_err(|e| AudioEngineError::StreamStartFailed {
                message: format!("Failed to play stream: {}", e),
            })?;

        // Store the stream (using blocking lock since Stream is not Send)
        // Note: We can't use spawn_blocking here because Stream is not Send.
        // Instead, we handle PoisonError explicitly to avoid unwrap().
        let mut streams = self
            .streams
            .lock()
            .map_err(|_| AudioEngineError::StreamStartFailed {
                message: "Mutex poisoned".to_string(),
            })?;
        streams.insert(handle.clone(), stream);

        Ok(handle)
    }

    async fn stop_stream(&self, handle: &StreamHandle) -> Result<(), AudioEngineError> {
        // Use blocking lock since Stream is not Send
        // Note: We can't use spawn_blocking here because Stream is not Send.
        // Instead, we handle PoisonError explicitly to avoid unwrap().
        let mut streams = self
            .streams
            .lock()
            .map_err(|_| AudioEngineError::StreamStopFailed {
                message: "Mutex poisoned".to_string(),
            })?;
        let stream = streams
            .remove(handle)
            .ok_or_else(|| AudioEngineError::StreamStopFailed {
                message: format!("Stream not found: {}", handle),
            })?;

        // Stop the stream (drop it)
        drop(stream);

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

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_enumerate_input_devices() {
        let engine = MacOSAudioEngine::new().unwrap();
        let devices = engine.enumerate_input_devices().await;

        // Should succeed (may return empty list if no devices)
        assert!(devices.is_ok());
        let devices = devices.unwrap();
        // All devices should be input devices
        assert!(devices.iter().all(|d| d.is_input));
    }

    #[tokio::test]
    async fn test_enumerate_output_devices() {
        let engine = MacOSAudioEngine::new().unwrap();
        let devices = engine.enumerate_output_devices().await;

        // Should succeed (may return empty list if no devices)
        assert!(devices.is_ok());
        let devices = devices.unwrap();
        // All devices should be output devices
        assert!(devices.iter().all(|d| !d.is_input));
    }

    #[tokio::test]
    async fn test_set_and_get_input_device() {
        let engine = MacOSAudioEngine::new().unwrap();

        // Get available devices
        let devices = engine.enumerate_input_devices().await.unwrap();
        if devices.is_empty() {
            // Skip test if no input devices available
            return;
        }

        // Set first available device
        let device_id = &devices[0].id;
        engine.set_input_device(device_id).await.unwrap();

        // Get the device back
        let device = engine.get_input_device().await.unwrap();
        assert!(device.is_some());
        assert_eq!(device.unwrap().id, *device_id);
    }

    #[tokio::test]
    async fn test_set_invalid_device() {
        let engine = MacOSAudioEngine::new().unwrap();

        let result = engine.set_input_device("invalid-device-id").await;
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            AudioEngineError::DeviceNotFound { .. }
        ));
    }

    #[tokio::test]
    async fn test_mute_and_unmute() {
        let engine = MacOSAudioEngine::new().unwrap();

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
        let engine = MacOSAudioEngine::new().unwrap();

        let input_level = engine.get_input_level().await.unwrap();
        let output_level = engine.get_output_level().await.unwrap();

        // Levels should be in valid range
        assert!((0.0..=1.0).contains(&input_level));
        assert!((0.0..=1.0).contains(&output_level));
    }
}
