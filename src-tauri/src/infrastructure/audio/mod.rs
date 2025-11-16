// Audio infrastructure module - Platform-specific audio implementations
// Contains cpal-based backends for platform-specific audio operations

// Platform-specific module declarations
#[cfg(target_os = "macos")]
pub mod macos;

#[cfg(target_os = "windows")]
pub mod windows;

#[cfg(target_os = "linux")]
pub mod linux;

// DeviceManager - Main entry point for creating platform-specific audio engines
pub mod device_manager;

// Re-exports for platform backends
#[cfg(target_os = "macos")]
pub use macos::MacOSAudioEngine;

// Re-export DeviceManager
pub use device_manager::create_audio_engine;
