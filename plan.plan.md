<!-- fbc00886-86d7-4f42-9901-04ac5b325feb 4b2986ff-22ab-4b52-a6e3-2c3489559881 -->
# AUD-5.1: AudioEngine Trait + Platform Abstraction

## Overview

Design the `AudioEngine` trait in the domain layer following the existing `CredentialStore` pattern. This trait abstracts platform-specific audio operations (device enumeration, streaming, mute) that will be implemented by `cpal`-based backends in AUD-5.2.

## Files to Create

1. **`/src-tauri/src/domain/traits/audio_engine.rs`** - AudioEngine trait definition
2. **`/src-tauri/src/infrastructure/audio/mod.rs`** - Platform abstraction module structure (no implementation yet)

## Files to Modify

1. **`/src-tauri/src/domain/traits/mod.rs`** - Export AudioEngine trait
2. **`/src-tauri/src/domain/mod.rs`** - Re-export AudioEngine
3. **`/src-tauri/src/domain/errors.rs`** - Add AudioEngineError type

## Implementation Details

### 1. AudioEngine Trait Design

The trait should support:

- **Device Enumeration**: List input/output audio devices
- **Device Selection**: Set active input/output devices
- **Audio Streaming**: Start/stop audio streams for calls
- **Mute Control**: Mute/unmute audio input
- **Audio Levels**: Get current input/output audio levels (for UI visualization)

### 2. Error Types

Create `AudioEngineError` in `domain/errors.rs` with variants:

- `DeviceNotFound` - Requested device ID doesn't exist
- `DeviceEnumerationFailed` - Failed to enumerate devices
- `StreamStartFailed` - Failed to start audio stream
- `StreamStopFailed` - Failed to stop audio stream
- `DeviceSwitchFailed` - Failed to switch devices
- `InvalidConfiguration` - Invalid audio configuration

### 3. Audio Device Entity (if needed)

The architecture mentions `audio_device.rs` entity. For AUD-5.1, we'll define a simple `AudioDevice` struct in the trait file or create a minimal entity. The full entity can be refined in AUD-5.3.

### 4. Platform Abstraction Module

Create `/src-tauri/src/infrastructure/audio/mod.rs` with:

- Platform-specific module declarations (`#[cfg(target_os = "macos")]`, etc.)
- Re-exports for platform backends
- Placeholder for future `DeviceManager` (AUD-5.2)

### 5. Mock Implementation

Include a mock `AudioEngine` implementation in the test module (similar to `MockCredentialStore`) for testing trait contract and usage in service layer tests.

## Design Considerations

1. **Async Pattern**: All operations async to support Tokio integration
2. **Send + Sync**: Trait must be `Send + Sync` for use in `Arc<dyn AudioEngine>`
3. **Device IDs**: Use `String` for device identifiers (platform-agnostic)
4. **Stream Handles**: Return stream handles/IDs for managing multiple streams
5. **Callback Bridge**: Design trait to work with `cpal`'s callback-based API (actual bridge implementation in AUD-5.2)

## Testing Strategy

- Unit tests for trait contract (using mock implementation)
- Test device enumeration, selection, stream lifecycle
- Test error conditions (invalid device IDs, stream failures)
- Test concurrent operations (multiple streams)

## Dependencies

- `async-trait` (already in Cargo.toml)
- No new dependencies for trait definition (cpal added in AUD-5.2)

## Success Criteria

- [x] AudioEngine trait defined with all required methods
- [x] AudioEngineError type defined with appropriate variants
- [x] Mock implementation included for testing
- [x] Platform abstraction module structure created
- [x] Trait exported from domain module
- [x] Unit tests passing with mock implementation
- [x] Documentation complete (doc comments for all methods)

### To-dos

- [x] Create AudioEngineError type in domain/errors.rs with all error variants
- [x] Design and implement AudioEngine trait in domain/traits/audio_engine.rs with device enumeration, selection, streaming, mute, and audio level methods
- [x] Define minimal AudioDevice struct (or reference) for trait methods
- [x] Implement MockAudioEngine in test module for trait contract testing
- [x] Create infrastructure/audio/mod.rs with platform-specific module structure and re-exports
- [x] Export AudioEngine trait from domain/traits/mod.rs and domain/mod.rs
- [x] Write comprehensive unit tests for AudioEngine trait using mock implementation

