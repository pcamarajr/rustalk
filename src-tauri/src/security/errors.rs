// Error types for security module

use thiserror::Error;

/// Security-related errors
#[derive(Error, Debug)]
pub enum SecurityError {
    #[error("Credential not found for username: {username}")]
    CredentialNotFound { username: String },

    #[error("Credential already exists for username: {username}")]
    CredentialAlreadyExists { username: String },

    #[error("Invalid input for field '{field}': {reason}")]
    InvalidInput { field: String, reason: String },

    #[error("Keyring operation failed: {0}")]
    KeyringError(String),

    #[error("Access denied: {0}")]
    AccessDenied(String),

    #[error("Internal error: {0}")]
    InternalError(String),
}

/// Result type alias for security operations
pub type SecurityResult<T> = Result<T, SecurityError>;

/// Convert SecurityError to String for Tauri commands
impl From<SecurityError> for String {
    fn from(err: SecurityError) -> String {
        err.to_string()
    }
}

/// Convert keyring::Error to SecurityError
impl From<keyring::Error> for SecurityError {
    fn from(err: keyring::Error) -> Self {
        match err {
            keyring::Error::NoEntry => SecurityError::CredentialNotFound {
                username: "unknown".to_string(),
            },
            keyring::Error::PlatformFailure(msg) => SecurityError::KeyringError(msg.to_string()),
            keyring::Error::NoStorageAccess(msg) => SecurityError::AccessDenied(msg.to_string()),
            _ => SecurityError::KeyringError(err.to_string()),
        }
    }
}
