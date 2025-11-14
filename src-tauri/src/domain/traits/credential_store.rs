// CredentialStore trait - Abstraction for secure credential storage
// Platform-specific implementations will be in infrastructure layer

use crate::domain::entities::Credentials;
use crate::domain::errors::CredentialStoreError;
use async_trait::async_trait;

/// Trait for secure credential storage operations
///
/// This trait abstracts platform-specific credential storage implementations
/// (e.g., macOS Keychain, Windows Credential Manager). All operations are async
/// to support non-blocking I/O operations.
#[async_trait]
pub trait CredentialStore: Send + Sync {
    /// Save credentials for a given account key
    ///
    /// # Arguments
    ///
    /// * `key` - Unique identifier for the account (e.g., username or account ID)
    /// * `credentials` - The credentials to store
    ///
    /// # Returns
    ///
    /// * `Ok(())` if credentials were saved successfully
    /// * `Err(CredentialStoreError)` if storage failed
    ///
    /// # Errors
    ///
    /// * `CredentialStoreError::StorageError` - Platform storage system error
    /// * `CredentialStoreError::InvalidKey` - Key is invalid or empty
    async fn save(&self, key: &str, credentials: &Credentials) -> Result<(), CredentialStoreError>;

    /// Load credentials for a given account key
    ///
    /// # Arguments
    ///
    /// * `key` - Unique identifier for the account
    ///
    /// # Returns
    ///
    /// * `Ok(Some(Credentials))` if credentials were found
    /// * `Ok(None)` if no credentials exist for the key
    /// * `Err(CredentialStoreError)` if retrieval failed
    ///
    /// # Errors
    ///
    /// * `CredentialStoreError::StorageError` - Platform storage system error
    /// * `CredentialStoreError::InvalidKey` - Key is invalid or empty
    async fn load(&self, key: &str) -> Result<Option<Credentials>, CredentialStoreError>;

    /// Delete credentials for a given account key
    ///
    /// # Arguments
    ///
    /// * `key` - Unique identifier for the account
    ///
    /// # Returns
    ///
    /// * `Ok(())` if credentials were deleted (or didn't exist)
    /// * `Err(CredentialStoreError)` if deletion failed
    ///
    /// # Errors
    ///
    /// * `CredentialStoreError::StorageError` - Platform storage system error
    /// * `CredentialStoreError::InvalidKey` - Key is invalid or empty
    async fn delete(&self, key: &str) -> Result<(), CredentialStoreError>;

    /// Check if credentials exist for a given account key
    ///
    /// # Arguments
    ///
    /// * `key` - Unique identifier for the account
    ///
    /// # Returns
    ///
    /// * `Ok(true)` if credentials exist
    /// * `Ok(false)` if no credentials exist
    /// * `Err(CredentialStoreError)` if check failed
    ///
    /// # Errors
    ///
    /// * `CredentialStoreError::StorageError` - Platform storage system error
    /// * `CredentialStoreError::InvalidKey` - Key is invalid or empty
    async fn exists(&self, key: &str) -> Result<bool, CredentialStoreError>;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::entities::{Credentials, TransportProtocol};
    use std::sync::Arc;
    use tokio::sync::Mutex;

    // Mock implementation for testing trait contract
    // Uses Arc<Mutex<>> for thread-safe interior mutability since async_trait requires Send + Sync
    struct MockCredentialStore {
        storage: Arc<Mutex<std::collections::HashMap<String, Credentials>>>,
    }

    impl MockCredentialStore {
        fn new() -> Self {
            Self {
                storage: Arc::new(Mutex::new(std::collections::HashMap::new())),
            }
        }
    }

    #[async_trait]
    impl CredentialStore for MockCredentialStore {
        async fn save(
            &self,
            key: &str,
            credentials: &Credentials,
        ) -> Result<(), CredentialStoreError> {
            if key.is_empty() {
                return Err(CredentialStoreError::InvalidKey {
                    key: key.to_string(),
                    reason: "Key cannot be empty".to_string(),
                });
            }
            let mut storage = self.storage.lock().await;
            storage.insert(key.to_string(), credentials.clone());
            Ok(())
        }

        async fn load(&self, key: &str) -> Result<Option<Credentials>, CredentialStoreError> {
            if key.is_empty() {
                return Err(CredentialStoreError::InvalidKey {
                    key: key.to_string(),
                    reason: "Key cannot be empty".to_string(),
                });
            }
            let storage = self.storage.lock().await;
            Ok(storage.get(key).cloned())
        }

        async fn delete(&self, key: &str) -> Result<(), CredentialStoreError> {
            if key.is_empty() {
                return Err(CredentialStoreError::InvalidKey {
                    key: key.to_string(),
                    reason: "Key cannot be empty".to_string(),
                });
            }
            let mut storage = self.storage.lock().await;
            storage.remove(key);
            Ok(())
        }

        async fn exists(&self, key: &str) -> Result<bool, CredentialStoreError> {
            if key.is_empty() {
                return Err(CredentialStoreError::InvalidKey {
                    key: key.to_string(),
                    reason: "Key cannot be empty".to_string(),
                });
            }
            let storage = self.storage.lock().await;
            Ok(storage.contains_key(key))
        }
    }

    #[tokio::test]
    async fn test_save_and_load() {
        let store = MockCredentialStore::new();
        let creds = Credentials::new(
            "sip.example.com".to_string(),
            5060,
            TransportProtocol::Udp,
            "user1".to_string(),
            "password123".to_string(),
        );

        store.save("account1", &creds).await.unwrap();
        let loaded = store.load("account1").await.unwrap();

        assert!(loaded.is_some());
        assert_eq!(loaded.unwrap(), creds);
    }

    #[tokio::test]
    async fn test_load_nonexistent() {
        let store = MockCredentialStore::new();
        let loaded = store.load("nonexistent").await.unwrap();

        assert!(loaded.is_none());
    }

    #[tokio::test]
    async fn test_delete() {
        let store = MockCredentialStore::new();
        let creds = Credentials::new(
            "sip.example.com".to_string(),
            5060,
            TransportProtocol::Udp,
            "user1".to_string(),
            "password123".to_string(),
        );

        store.save("account1", &creds).await.unwrap();
        assert!(store.exists("account1").await.unwrap());

        store.delete("account1").await.unwrap();
        assert!(!store.exists("account1").await.unwrap());
    }

    #[tokio::test]
    async fn test_exists() {
        let store = MockCredentialStore::new();
        let creds = Credentials::new(
            "sip.example.com".to_string(),
            5060,
            TransportProtocol::Udp,
            "user1".to_string(),
            "password123".to_string(),
        );

        assert!(!store.exists("account1").await.unwrap());
        store.save("account1", &creds).await.unwrap();
        assert!(store.exists("account1").await.unwrap());
    }

    #[tokio::test]
    async fn test_invalid_key() {
        let store = MockCredentialStore::new();
        let creds = Credentials::new(
            "sip.example.com".to_string(),
            5060,
            TransportProtocol::Udp,
            "user1".to_string(),
            "password123".to_string(),
        );

        let result = store.save("", &creds).await;
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            CredentialStoreError::InvalidKey { .. }
        ));

        let result = store.load("").await;
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            CredentialStoreError::InvalidKey { .. }
        ));
    }

    #[tokio::test]
    async fn test_multiple_accounts() {
        let store = MockCredentialStore::new();
        let creds1 = Credentials::new(
            "sip.example.com".to_string(),
            5060,
            TransportProtocol::Udp,
            "user1".to_string(),
            "password1".to_string(),
        );
        let creds2 = Credentials::new(
            "sip.other.com".to_string(),
            5061,
            TransportProtocol::Tls,
            "user2".to_string(),
            "password2".to_string(),
        );

        store.save("account1", &creds1).await.unwrap();
        store.save("account2", &creds2).await.unwrap();

        let loaded1 = store.load("account1").await.unwrap().unwrap();
        let loaded2 = store.load("account2").await.unwrap().unwrap();

        assert_eq!(loaded1, creds1);
        assert_eq!(loaded2, creds2);
        assert_ne!(loaded1, loaded2);
    }
}
