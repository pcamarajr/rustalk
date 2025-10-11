// RUSTALK Security Error Types
//
// Comprehensive error types for credential management and security operations.

use thiserror::Error;

/// Keychain-specific errors
#[derive(Error, Debug, Clone)]
pub enum KeychainError {
    /// Keychain service unavailable
    #[error("Keychain service unavailable")]
    Unavailable,

    /// Permission denied by OS
    #[error("Permission denied")]
    PermissionDenied,

    /// Generic keychain error
    #[error("Keychain error: {0}")]
    Other(String),
}

impl KeychainError {
    /// Convert keyring::Error to KeychainError
    pub fn from_keyring_error(err: keyring::Error) -> Self {
        match err {
            keyring::Error::NoEntry => KeychainError::Other("No entry found".to_string()),
            keyring::Error::NoStorageAccess(_) => KeychainError::PermissionDenied,
            keyring::Error::PlatformFailure(e) => KeychainError::Other(e.to_string()),
            _ => KeychainError::Unavailable,
        }
    }
}

/// Credential-related errors
#[derive(Error, Debug, Clone)]
pub enum CredentialError {
    /// Credential not found
    #[error("Credential not found")]
    NotFound,

    /// Credential already exists
    #[error("Credential already exists")]
    AlreadyExists,

    /// Validation error
    #[error("Validation error: {0}")]
    Validation(String),

    /// Keychain error
    #[error("Keychain error: {0}")]
    Keychain(#[from] KeychainError),

    /// Internal error
    #[error("Internal error: {0}")]
    Internal(String),
}

// Legacy aliases for backward compatibility
pub type SecurityError = CredentialError;
pub type SecurityResult<T> = Result<T, CredentialError>;

// Implement conversion to String for Tauri commands
impl From<CredentialError> for String {
    fn from(err: CredentialError) -> String {
        err.to_string()
    }
}
