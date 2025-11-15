// Domain error types for credential storage operations

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
