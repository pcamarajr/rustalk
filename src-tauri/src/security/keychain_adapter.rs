// Keychain adapter using keyring crate
//
// CRITICAL: All keyring operations MUST be wrapped in tokio::task::spawn_blocking
// because the keyring crate is synchronous and will deadlock the Tokio runtime.

use crate::security::error::{CredentialError, KeychainError};
use tokio::task;

/// Keychain adapter (internal implementation)
pub struct KeychainAdapter {
    service_name: String,
}

impl KeychainAdapter {
    /// Create new adapter with service name prefix
    pub fn new(service_name: impl Into<String>) -> Self {
        Self {
            service_name: service_name.into(),
        }
    }

    /// Store credentials in platform keychain (Tokio-safe)
    pub async fn store(&self, account: &str, password: &str) -> Result<(), CredentialError> {
        let service = self.service_name.clone();
        let account = account.to_string();
        let password = password.to_string();

        task::spawn_blocking(move || {
            let entry = keyring::Entry::new(&service, &account)
                .map_err(|_| CredentialError::Keychain(KeychainError::Unavailable))?;

            entry
                .set_password(&password)
                .map_err(|e| CredentialError::Keychain(KeychainError::from_keyring_error(e)))?;

            Ok(())
        })
        .await
        .map_err(|_| CredentialError::Keychain(KeychainError::Unavailable))?
    }

    /// Retrieve credentials from platform keychain (Tokio-safe)
    pub async fn get(&self, account: &str) -> Result<String, CredentialError> {
        let service = self.service_name.clone();
        let account = account.to_string();

        task::spawn_blocking(move || {
            let entry = keyring::Entry::new(&service, &account)
                .map_err(|_| CredentialError::Keychain(KeychainError::Unavailable))?;

            entry
                .get_password()
                .map_err(|e| match e {
                    keyring::Error::NoEntry => CredentialError::NotFound,
                    _ => CredentialError::Keychain(KeychainError::from_keyring_error(e)),
                })
        })
        .await
        .map_err(|_| CredentialError::Keychain(KeychainError::Unavailable))?
    }

    /// Delete credentials from platform keychain (Tokio-safe)
    pub async fn delete(&self, account: &str) -> Result<(), CredentialError> {
        let service = self.service_name.clone();
        let account = account.to_string();

        task::spawn_blocking(move || {
            let entry = keyring::Entry::new(&service, &account)
                .map_err(|_| CredentialError::Keychain(KeychainError::Unavailable))?;

            entry
                .delete_credential()
                .map_err(|e| match e {
                    keyring::Error::NoEntry => CredentialError::NotFound,
                    _ => CredentialError::Keychain(KeychainError::from_keyring_error(e)),
                })
        })
        .await
        .map_err(|_| CredentialError::Keychain(KeychainError::Unavailable))?
    }

    /// Check if credentials exist (Tokio-safe)
    pub async fn exists(&self, account: &str) -> Result<bool, CredentialError> {
        match self.get(account).await {
            Ok(_) => Ok(true),
            Err(CredentialError::NotFound) => Ok(false),
            Err(e) => Err(e),
        }
    }
}
