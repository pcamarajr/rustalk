// Domain error types for credential storage and audio engine operations

use thiserror::Error;

/// Errors that can occur during credential storage operations
#[derive(Debug, Error, Clone, PartialEq, Eq)]
pub enum CredentialStoreError {
    /// Storage system error (platform-specific)
    #[error("Storage error: {message}")]
    StorageError {
        /// Error message
        message: String,
    },

    /// Invalid key provided
    #[error("Invalid key '{key}': {reason}")]
    InvalidKey {
        /// The invalid key
        key: String,
        /// Reason why the key is invalid
        reason: String,
    },

    /// Credentials not found (for operations that require existing credentials)
    #[error("Credentials not found for key: {key}")]
    NotFound {
        /// The key that was not found
        key: String,
    },
}

/// Errors that can occur during audio engine operations
#[derive(Debug, Error, Clone, PartialEq, Eq)]
pub enum AudioEngineError {
    /// Requested device ID doesn't exist
    #[error("Device not found: {device_id}")]
    DeviceNotFound {
        /// The device ID that was not found
        device_id: String,
    },

    /// Failed to enumerate audio devices
    #[error("Device enumeration failed: {message}")]
    DeviceEnumerationFailed {
        /// Error message
        message: String,
    },

    /// Failed to start audio stream
    #[error("Stream start failed: {message}")]
    StreamStartFailed {
        /// Error message
        message: String,
    },

    /// Failed to stop audio stream
    #[error("Stream stop failed: {message}")]
    StreamStopFailed {
        /// Error message
        message: String,
    },

    /// Failed to switch audio devices
    #[error("Device switch failed: {message}")]
    DeviceSwitchFailed {
        /// Error message
        message: String,
    },

    /// Invalid audio configuration
    #[error("Invalid configuration: {message}")]
    InvalidConfiguration {
        /// Error message
        message: String,
    },
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_storage_error_display() {
        let error = CredentialStoreError::StorageError {
            message: "Keychain access denied".to_string(),
        };
        assert_eq!(error.to_string(), "Storage error: Keychain access denied");
    }

    #[test]
    fn test_invalid_key_display() {
        let error = CredentialStoreError::InvalidKey {
            key: "".to_string(),
            reason: "Key cannot be empty".to_string(),
        };
        assert_eq!(error.to_string(), "Invalid key '': Key cannot be empty");
    }

    #[test]
    fn test_not_found_display() {
        let error = CredentialStoreError::NotFound {
            key: "account1".to_string(),
        };
        assert_eq!(error.to_string(), "Credentials not found for key: account1");
    }

    #[test]
    fn test_error_equality() {
        let error1 = CredentialStoreError::StorageError {
            message: "Error".to_string(),
        };
        let error2 = CredentialStoreError::StorageError {
            message: "Error".to_string(),
        };
        let error3 = CredentialStoreError::StorageError {
            message: "Different".to_string(),
        };

        assert_eq!(error1, error2);
        assert_ne!(error1, error3);
    }
}

#[cfg(test)]
mod audio_engine_error_tests {
    use super::*;

    #[test]
    fn test_device_not_found_display() {
        let error = AudioEngineError::DeviceNotFound {
            device_id: "device-123".to_string(),
        };
        assert_eq!(error.to_string(), "Device not found: device-123");
    }

    #[test]
    fn test_device_enumeration_failed_display() {
        let error = AudioEngineError::DeviceEnumerationFailed {
            message: "Permission denied".to_string(),
        };
        assert_eq!(error.to_string(), "Device enumeration failed: Permission denied");
    }

    #[test]
    fn test_stream_start_failed_display() {
        let error = AudioEngineError::StreamStartFailed {
            message: "Device busy".to_string(),
        };
        assert_eq!(error.to_string(), "Stream start failed: Device busy");
    }

    #[test]
    fn test_stream_stop_failed_display() {
        let error = AudioEngineError::StreamStopFailed {
            message: "Stream not found".to_string(),
        };
        assert_eq!(error.to_string(), "Stream stop failed: Stream not found");
    }

    #[test]
    fn test_device_switch_failed_display() {
        let error = AudioEngineError::DeviceSwitchFailed {
            message: "Device unavailable".to_string(),
        };
        assert_eq!(error.to_string(), "Device switch failed: Device unavailable");
    }

    #[test]
    fn test_invalid_configuration_display() {
        let error = AudioEngineError::InvalidConfiguration {
            message: "Invalid sample rate".to_string(),
        };
        assert_eq!(error.to_string(), "Invalid configuration: Invalid sample rate");
    }

    #[test]
    fn test_audio_engine_error_equality() {
        let error1 = AudioEngineError::DeviceNotFound {
            device_id: "device-1".to_string(),
        };
        let error2 = AudioEngineError::DeviceNotFound {
            device_id: "device-1".to_string(),
        };
        let error3 = AudioEngineError::DeviceNotFound {
            device_id: "device-2".to_string(),
        };

        assert_eq!(error1, error2);
        assert_ne!(error1, error3);
    }
}
