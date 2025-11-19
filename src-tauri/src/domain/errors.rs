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

/// Errors that can occur during SIP operations
#[derive(Debug, Error, Clone, PartialEq, Eq)]
pub enum SipError {
    /// Failed to parse SIP message
    #[error("Parse error: {message}")]
    ParseError {
        /// Error message describing the parse failure
        message: String,
    },

    /// Invalid SIP message structure
    #[error("Invalid message: {reason}")]
    InvalidMessage {
        /// Reason why the message is invalid
        reason: String,
    },

    /// Unsupported SIP method
    #[error("Unsupported method: {method}")]
    UnsupportedMethod {
        /// The unsupported method name
        method: String,
    },

    /// Required header missing
    #[error("Missing required header: {header}")]
    MissingHeader {
        /// The name of the missing header
        header: String,
    },

    /// Network connection failure
    #[error("Connection error: {message}")]
    ConnectionError {
        /// Error message describing the connection failure
        message: String,
    },

    /// Transport layer error
    #[error("Transport error: {message}")]
    TransportError {
        /// Error message describing the transport failure
        message: String,
    },

    /// TLS handshake or certificate error
    #[error("TLS error: {message}")]
    TlsError {
        /// Error message describing the TLS failure
        message: String,
    },

    /// Operation timeout
    #[error("Timeout error: {message}")]
    TimeoutError {
        /// Error message describing the timeout
        message: String,
    },
}

/// Errors that can occur during RTP operations
#[derive(Debug, Error, Clone, PartialEq, Eq)]
pub enum RtpError {
    /// Failed to bind RTP socket
    #[error("Socket bind failed: {message}")]
    SocketBindFailed {
        /// Error message
        message: String,
    },

    /// Failed to send RTP packet
    #[error("Packet send failed: {message}")]
    SendFailed {
        /// Error message
        message: String,
    },

    /// Failed to receive RTP packet
    #[error("Packet receive failed: {message}")]
    ReceiveFailed {
        /// Error message
        message: String,
    },

    /// Invalid RTP packet format
    #[error("Invalid packet format: {message}")]
    InvalidPacket {
        /// Error message
        message: String,
    },

    /// Codec encoding/decoding error
    #[error("Codec error: {message}")]
    CodecError {
        /// Error message
        message: String,
    },

    /// Session already started
    #[error("Session already started")]
    SessionAlreadyStarted,

    /// Session not started
    #[error("Session not started")]
    SessionNotStarted,

    /// Invalid configuration
    #[error("Invalid configuration: {message}")]
    InvalidConfiguration {
        /// Error message
        message: String,
    },
}

impl From<RtpError> for SipError {
    fn from(err: RtpError) -> Self {
        match err {
            RtpError::SocketBindFailed { message } => SipError::TransportError {
                message: format!("RTP socket bind failed: {}", message),
            },
            RtpError::SendFailed { message } => SipError::TransportError {
                message: format!("RTP send failed: {}", message),
            },
            RtpError::ReceiveFailed { message } => SipError::TransportError {
                message: format!("RTP receive failed: {}", message),
            },
            RtpError::InvalidPacket { message } => SipError::InvalidMessage {
                reason: format!("Invalid RTP packet: {}", message),
            },
            RtpError::CodecError { message } => SipError::InvalidMessage {
                reason: format!("RTP codec error: {}", message),
            },
            RtpError::SessionAlreadyStarted => SipError::InvalidMessage {
                reason: "RTP session already started".to_string(),
            },
            RtpError::SessionNotStarted => SipError::InvalidMessage {
                reason: "RTP session not started".to_string(),
            },
            RtpError::InvalidConfiguration { message } => SipError::InvalidMessage {
                reason: format!("RTP configuration error: {}", message),
            },
        }
    }
}

impl From<std::io::Error> for SipError {
    fn from(err: std::io::Error) -> Self {
        SipError::TransportError {
            message: format!("IO error: {}", err),
        }
    }
}

impl From<SipError> for CommandError {
    fn from(err: SipError) -> Self {
        match err {
            SipError::InvalidMessage { reason } => CommandError::ServiceError {
                message: format!("Invalid message: {}", reason),
            },
            SipError::ConnectionError { message } => CommandError::ServiceError {
                message: format!("Connection error: {}", message),
            },
            SipError::TransportError { message } => CommandError::ServiceError {
                message: format!("Transport error: {}", message),
            },
            SipError::ParseError { message } => CommandError::ServiceError {
                message: format!("Parse error: {}", message),
            },
            SipError::UnsupportedMethod { method } => CommandError::ServiceError {
                message: format!("Unsupported method: {}", method),
            },
            SipError::MissingHeader { header } => CommandError::ServiceError {
                message: format!("Missing required header: {}", header),
            },
            SipError::TlsError { message } => CommandError::ServiceError {
                message: format!("TLS error: {}", message),
            },
            SipError::TimeoutError { message } => CommandError::ServiceError {
                message: format!("Timeout error: {}", message),
            },
        }
    }
}

impl From<AudioEngineError> for CommandError {
    fn from(err: AudioEngineError) -> Self {
        match err {
            AudioEngineError::DeviceNotFound { device_id } => CommandError::InvalidArgument {
                argument: "device_id".to_string(),
                reason: format!("Device not found: {}", device_id),
            },
            AudioEngineError::DeviceEnumerationFailed { message } => CommandError::ServiceError {
                message: format!("Device enumeration failed: {}", message),
            },
            AudioEngineError::StreamStartFailed { message } => CommandError::ServiceError {
                message: format!("Stream start failed: {}", message),
            },
            AudioEngineError::StreamStopFailed { message } => CommandError::ServiceError {
                message: format!("Stream stop failed: {}", message),
            },
            AudioEngineError::DeviceSwitchFailed { message } => CommandError::ServiceError {
                message: format!("Device switch failed: {}", message),
            },
            AudioEngineError::InvalidConfiguration { message } => CommandError::ServiceError {
                message: format!("Invalid configuration: {}", message),
            },
        }
    }
}

/// Errors that can occur during Tauri command execution
#[derive(Debug, Error, Clone, PartialEq, Eq, serde::Serialize)]
pub enum CommandError {
    /// Input validation failures
    #[error("Validation error for field '{field}': {message}")]
    ValidationError {
        /// The field that failed validation
        field: String,
        /// Error message describing the validation failure
        message: String,
    },

    /// Invalid parameter values
    #[error("Invalid argument '{argument}': {reason}")]
    InvalidArgument {
        /// The argument that is invalid
        argument: String,
        /// Reason why the argument is invalid
        reason: String,
    },

    /// Required parameters missing
    #[error("Missing required argument: {argument}")]
    MissingArgument {
        /// The argument that is missing
        argument: String,
    },

    /// Service layer errors (for future use)
    #[error("Service error: {message}")]
    ServiceError {
        /// Error message from the service layer
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

    #[test]
    fn test_validation_error_display() {
        let error = CommandError::ValidationError {
            field: "name".to_string(),
            message: "Name cannot be empty".to_string(),
        };
        assert_eq!(
            error.to_string(),
            "Validation error for field 'name': Name cannot be empty"
        );
    }

    #[test]
    fn test_invalid_argument_display() {
        let error = CommandError::InvalidArgument {
            argument: "port".to_string(),
            reason: "Port must be between 1 and 65535".to_string(),
        };
        assert_eq!(
            error.to_string(),
            "Invalid argument 'port': Port must be between 1 and 65535"
        );
    }

    #[test]
    fn test_missing_argument_display() {
        let error = CommandError::MissingArgument {
            argument: "hostname".to_string(),
        };
        assert_eq!(error.to_string(), "Missing required argument: hostname");
    }

    #[test]
    fn test_service_error_display() {
        let error = CommandError::ServiceError {
            message: "Connection failed".to_string(),
        };
        assert_eq!(error.to_string(), "Service error: Connection failed");
    }

    #[test]
    fn test_command_error_equality() {
        let error1 = CommandError::ValidationError {
            field: "name".to_string(),
            message: "Error".to_string(),
        };
        let error2 = CommandError::ValidationError {
            field: "name".to_string(),
            message: "Error".to_string(),
        };
        let error3 = CommandError::ValidationError {
            field: "name".to_string(),
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
        assert_eq!(
            error.to_string(),
            "Device enumeration failed: Permission denied"
        );
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
        assert_eq!(
            error.to_string(),
            "Device switch failed: Device unavailable"
        );
    }

    #[test]
    fn test_invalid_configuration_display() {
        let error = AudioEngineError::InvalidConfiguration {
            message: "Invalid sample rate".to_string(),
        };
        assert_eq!(
            error.to_string(),
            "Invalid configuration: Invalid sample rate"
        );
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

#[cfg(test)]
mod sip_error_tests {
    use super::*;

    #[test]
    fn test_parse_error_display() {
        let error = SipError::ParseError {
            message: "Invalid start line".to_string(),
        };
        assert_eq!(error.to_string(), "Parse error: Invalid start line");
    }

    #[test]
    fn test_invalid_message_display() {
        let error = SipError::InvalidMessage {
            reason: "Missing request line".to_string(),
        };
        assert_eq!(error.to_string(), "Invalid message: Missing request line");
    }

    #[test]
    fn test_unsupported_method_display() {
        let error = SipError::UnsupportedMethod {
            method: "REFER".to_string(),
        };
        assert_eq!(error.to_string(), "Unsupported method: REFER");
    }

    #[test]
    fn test_missing_header_display() {
        let error = SipError::MissingHeader {
            header: "From".to_string(),
        };
        assert_eq!(error.to_string(), "Missing required header: From");
    }

    #[test]
    fn test_sip_error_equality() {
        let error1 = SipError::ParseError {
            message: "Error".to_string(),
        };
        let error2 = SipError::ParseError {
            message: "Error".to_string(),
        };
        let error3 = SipError::ParseError {
            message: "Different".to_string(),
        };

        assert_eq!(error1, error2);
        assert_ne!(error1, error3);
    }
}
