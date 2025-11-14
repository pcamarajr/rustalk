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
