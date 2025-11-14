// macOS Keychain implementation of CredentialStore trait

use crate::domain::entities::Credentials;
use crate::domain::errors::CredentialStoreError;
use crate::domain::traits::CredentialStore;
use async_trait::async_trait;
use keyring::Entry;

/// Service name for all Keychain entries
const SERVICE_NAME: &str = "rustalk";

/// Map keyring errors to domain errors
fn map_keyring_error(error: keyring::Error, operation: &str) -> CredentialStoreError {
    match error {
        keyring::Error::NoEntry => {
            // This should not happen in save/delete operations, but handle gracefully
            CredentialStoreError::StorageError {
                message: format!("{}: No entry found", operation),
            }
        }
        keyring::Error::Ambiguous(msg) => CredentialStoreError::StorageError {
            message: format!("{}: Ambiguous entry - {}", operation, msg),
        },
        keyring::Error::PlatformFailure(msg) => CredentialStoreError::StorageError {
            message: format!("{}: Platform failure - {}", operation, msg),
        },
        _ => CredentialStoreError::StorageError {
            message: format!("{}: {}", operation, error),
        },
    }
}

/// macOS Keychain-based credential store
///
/// This implementation uses the macOS Keychain Services API via the `keyring` crate
/// to securely store SIP account credentials. All operations are wrapped in async
/// tasks to avoid blocking the runtime.
#[cfg(target_os = "macos")]
pub struct KeychainCredentialStore;

#[cfg(target_os = "macos")]
impl KeychainCredentialStore {
    /// Create a new KeychainCredentialStore instance
    pub fn new() -> Self {
        Self
    }

    /// Validate that a key is not empty
    fn validate_key(key: &str) -> Result<(), CredentialStoreError> {
        if key.is_empty() {
            return Err(CredentialStoreError::InvalidKey {
                key: key.to_string(),
                reason: "Key cannot be empty".to_string(),
            });
        }
        Ok(())
    }
}

#[cfg(target_os = "macos")]
#[async_trait]
impl CredentialStore for KeychainCredentialStore {
    async fn save(&self, key: &str, credentials: &Credentials) -> Result<(), CredentialStoreError> {
        Self::validate_key(key)?;

        let key = key.to_string();
        let credentials_json = serde_json::to_string(credentials).map_err(|e| {
            CredentialStoreError::StorageError {
                message: format!("Failed to serialize credentials: {}", e),
            }
        })?;

        let service = SERVICE_NAME.to_string();
        let credentials_json_clone = credentials_json.clone();

        tokio::task::spawn_blocking(move || {
            let entry = Entry::new(&service, &key).map_err(|e| {
                CredentialStoreError::StorageError {
                    message: format!("Failed to create keyring entry: {}", e),
                }
            })?;

            entry.set_password(&credentials_json_clone).map_err(|e| {
                map_keyring_error(e, "save")
            })
        })
        .await
        .map_err(|e| CredentialStoreError::StorageError {
            message: format!("Task join error: {}", e),
        })?
    }

    async fn load(&self, key: &str) -> Result<Option<Credentials>, CredentialStoreError> {
        Self::validate_key(key)?;

        let key = key.to_string();
        let service = SERVICE_NAME.to_string();

        tokio::task::spawn_blocking(move || {
            let entry = Entry::new(&service, &key).map_err(|e| {
                CredentialStoreError::StorageError {
                    message: format!("Failed to create keyring entry: {}", e),
                }
            })?;

            match entry.get_password() {
                Ok(password_json) => {
                    serde_json::from_str(&password_json).map_err(|e| {
                        CredentialStoreError::StorageError {
                            message: format!("Failed to deserialize credentials: {}", e),
                        }
                    })
                    .map(Some)
                }
                Err(keyring::Error::NoEntry) => Ok(None),
                Err(e) => Err(map_keyring_error(e, "load")),
            }
        })
        .await
        .map_err(|e| CredentialStoreError::StorageError {
            message: format!("Task join error: {}", e),
        })?
    }

    async fn delete(&self, key: &str) -> Result<(), CredentialStoreError> {
        Self::validate_key(key)?;

        let key = key.to_string();
        let service = SERVICE_NAME.to_string();

        tokio::task::spawn_blocking(move || {
            let entry = Entry::new(&service, &key).map_err(|e| {
                CredentialStoreError::StorageError {
                    message: format!("Failed to create keyring entry: {}", e),
                }
            })?;

            match entry.delete_password() {
                Ok(()) => Ok(()),
                Err(keyring::Error::NoEntry) => Ok(()), // Idempotent - already deleted
                Err(e) => Err(map_keyring_error(e, "delete")),
            }
        })
        .await
        .map_err(|e| CredentialStoreError::StorageError {
            message: format!("Task join error: {}", e),
        })?
    }

    async fn exists(&self, key: &str) -> Result<bool, CredentialStoreError> {
        Self::validate_key(key)?;
        let result = self.load(key).await?;
        Ok(result.is_some())
    }
}

#[cfg(target_os = "macos")]
impl Default for KeychainCredentialStore {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
#[cfg(target_os = "macos")]
mod tests {
    use super::*;
    use crate::domain::entities::TransportProtocol;

    fn create_test_credentials() -> Credentials {
        Credentials::new(
            "sip.example.com".to_string(),
            5060,
            TransportProtocol::Udp,
            "testuser".to_string(),
            "testpassword".to_string(),
        )
    }

    fn create_unique_key() -> String {
        use std::time::{SystemTime, UNIX_EPOCH};
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        format!("test_account_{}", timestamp)
    }

    #[tokio::test]
    async fn test_save_and_load() {
        let store = KeychainCredentialStore::new();
        let key = create_unique_key();
        let credentials = create_test_credentials();

        // Save credentials
        store.save(&key, &credentials).await.unwrap();

        // Load credentials back
        let loaded = store.load(&key).await.unwrap();

        assert!(loaded.is_some());
        let loaded_creds = loaded.unwrap();
        assert_eq!(loaded_creds, credentials);

        // Cleanup
        store.delete(&key).await.unwrap();
    }

    #[tokio::test]
    async fn test_load_nonexistent() {
        let store = KeychainCredentialStore::new();
        let key = create_unique_key();

        let loaded = store.load(&key).await.unwrap();
        assert!(loaded.is_none());
    }

    #[tokio::test]
    async fn test_delete() {
        let store = KeychainCredentialStore::new();
        let key = create_unique_key();
        let credentials = create_test_credentials();

        // Save credentials
        store.save(&key, &credentials).await.unwrap();

        // Verify they exist
        assert!(store.exists(&key).await.unwrap());

        // Delete credentials
        store.delete(&key).await.unwrap();

        // Verify they're gone
        assert!(!store.exists(&key).await.unwrap());
        let loaded = store.load(&key).await.unwrap();
        assert!(loaded.is_none());
    }

    #[tokio::test]
    async fn test_delete_nonexistent() {
        let store = KeychainCredentialStore::new();
        let key = create_unique_key();

        // Deleting non-existent key should succeed (idempotent)
        store.delete(&key).await.unwrap();
    }

    #[tokio::test]
    async fn test_exists() {
        let store = KeychainCredentialStore::new();
        let key = create_unique_key();
        let credentials = create_test_credentials();

        // Initially doesn't exist
        assert!(!store.exists(&key).await.unwrap());

        // Save credentials
        store.save(&key, &credentials).await.unwrap();

        // Now exists
        assert!(store.exists(&key).await.unwrap());

        // Cleanup
        store.delete(&key).await.unwrap();

        // No longer exists
        assert!(!store.exists(&key).await.unwrap());
    }

    #[tokio::test]
    async fn test_multiple_accounts() {
        let store = KeychainCredentialStore::new();
        let key1 = create_unique_key();
        let key2 = create_unique_key();
        let creds1 = Credentials::new(
            "sip1.example.com".to_string(),
            5060,
            TransportProtocol::Udp,
            "user1".to_string(),
            "pass1".to_string(),
        );
        let creds2 = Credentials::new(
            "sip2.example.com".to_string(),
            5061,
            TransportProtocol::Tcp,
            "user2".to_string(),
            "pass2".to_string(),
        );

        // Save both accounts
        store.save(&key1, &creds1).await.unwrap();
        store.save(&key2, &creds2).await.unwrap();

        // Load and verify both
        let loaded1 = store.load(&key1).await.unwrap().unwrap();
        let loaded2 = store.load(&key2).await.unwrap().unwrap();

        assert_eq!(loaded1, creds1);
        assert_eq!(loaded2, creds2);

        // Cleanup
        store.delete(&key1).await.unwrap();
        store.delete(&key2).await.unwrap();
    }

    #[tokio::test]
    async fn test_empty_key_error() {
        let store = KeychainCredentialStore::new();
        let credentials = create_test_credentials();

        // Empty key should fail
        let result = store.save("", &credentials).await;
        assert!(result.is_err());
        match result.unwrap_err() {
            CredentialStoreError::InvalidKey { .. } => {}
            _ => panic!("Expected InvalidKey error"),
        }

        let result = store.load("").await;
        assert!(result.is_err());
        match result.unwrap_err() {
            CredentialStoreError::InvalidKey { .. } => {}
            _ => panic!("Expected InvalidKey error"),
        }

        let result = store.delete("").await;
        assert!(result.is_err());
        match result.unwrap_err() {
            CredentialStoreError::InvalidKey { .. } => {}
            _ => panic!("Expected InvalidKey error"),
        }

        let result = store.exists("").await;
        assert!(result.is_err());
        match result.unwrap_err() {
            CredentialStoreError::InvalidKey { .. } => {}
            _ => panic!("Expected InvalidKey error"),
        }
    }

    #[tokio::test]
    async fn test_data_integrity() {
        let store = KeychainCredentialStore::new();
        let key = create_unique_key();
        let credentials = Credentials::new(
            "sip.test.com".to_string(),
            5060,
            TransportProtocol::Tls,
            "testuser".to_string(),
            "securepassword123".to_string(),
        );

        // Save
        store.save(&key, &credentials).await.unwrap();

        // Load and verify all fields
        let loaded = store.load(&key).await.unwrap().unwrap();

        assert_eq!(loaded.server, "sip.test.com");
        assert_eq!(loaded.port, 5060);
        assert_eq!(loaded.protocol, TransportProtocol::Tls);
        assert_eq!(loaded.username, "testuser");
        assert_eq!(loaded.password, "securepassword123");

        // Cleanup
        store.delete(&key).await.unwrap();
    }
}

