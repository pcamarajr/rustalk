// DeviceManager - Main entry point for platform-specific audio implementations
// Provides a unified interface to create platform-specific AudioEngine instances

use crate::domain::errors::AudioEngineError;
use crate::domain::traits::audio_engine::AudioEngine;

#[cfg(target_os = "macos")]
use super::macos::MacOSAudioEngine;

/// Create a platform-specific audio engine instance
///
/// This function returns the appropriate AudioEngine implementation for the current platform.
/// On macOS, it returns a MacOSAudioEngine using cpal.
///
/// # Returns
///
/// * `Ok(Box<dyn AudioEngine>)` - Platform-specific audio engine
/// * `Err(AudioEngineError)` - Failed to create audio engine
pub fn create_audio_engine() -> Result<Box<dyn AudioEngine>, AudioEngineError> {
    #[cfg(target_os = "macos")]
    {
        MacOSAudioEngine::new().map(|engine| Box::new(engine) as Box<dyn AudioEngine>)
    }

    #[cfg(not(target_os = "macos"))]
    {
        Err(AudioEngineError::DeviceEnumerationFailed {
            message: format!(
                "Audio engine not implemented for platform: {}",
                std::env::consts::OS
            ),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[cfg(target_os = "macos")]
    async fn test_create_audio_engine_macos() {
        let engine = create_audio_engine();
        assert!(engine.is_ok());
    }

    #[tokio::test]
    #[cfg(not(target_os = "macos"))]
    async fn test_create_audio_engine_unsupported() {
        let engine = create_audio_engine();
        assert!(engine.is_err());
    }
}
