// Keychain provider implementation using the keyring crate
//
// CRITICAL: All keyring operations MUST be wrapped in tokio::task::spawn_blocking
// because the keyring crate is synchronous and will deadlock the Tokio runtime
// if called directly from async context.

use crate::security::{
    errors::{SecurityError, SecurityResult},
    validation::Validator,
    SIP_SERVICE_NAME,
};
use keyring::Entry;

/// Trait for platform-agnostic keychain operations
#[async_trait::async_trait]
pub trait KeychainProvider: Send + Sync {
    /// Store credentials securely in the platform keychain
    async fn store_credentials(&self, username: &str, password: &str) -> SecurityResult<()>;

    /// Retrieve credentials from the platform keychain
    async fn get_credentials(&self, username: &str) -> SecurityResult<String>;

    /// Delete credentials from the platform keychain
    async fn delete_credentials(&self, username: &str) -> SecurityResult<()>;

    /// Check if credentials exist for the given username
    async fn check_exists(&self, username: &str) -> SecurityResult<bool>;

    /// List all account usernames (metadata only, no passwords)
    /// Note: This is a best-effort operation and may not be supported on all platforms
    async fn list_accounts(&self) -> SecurityResult<Vec<String>>;
}

/// Concrete implementation of KeychainProvider using the keyring crate
pub struct CredentialStore {
    service_name: String,
}

impl CredentialStore {
    /// Create a new CredentialStore with the default service name
    pub fn new() -> Self {
        Self {
            service_name: SIP_SERVICE_NAME.to_string(),
        }
    }

    /// Create a new CredentialStore with a custom service name
    pub fn with_service_name(service_name: String) -> Self {
        Self { service_name }
    }

    /// Create a keyring entry for the given username
    ///
    /// This is a helper method that creates the Entry object.
    /// The Entry object handles platform-specific keychain access.
    fn create_entry(&self, username: &str) -> Result<Entry, keyring::Error> {
        Entry::new(&self.service_name, username)
    }
}

impl Default for CredentialStore {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait::async_trait]
impl KeychainProvider for CredentialStore {
    /// Store credentials securely in the platform keychain
    ///
    /// # Security
    /// - Validates username and password before storage
    /// - Uses platform-native encryption (Keychain/Credential Manager)
    /// - Overwrites existing credentials if they exist
    ///
    /// # Platform Behavior
    /// - macOS: Stored in login keychain with application-specific ACL
    /// - Windows: Stored in Credential Manager with user-level persistence
    ///
    /// # Errors
    /// - `InvalidInput`: If validation fails
    /// - `KeyringError`: If platform keychain operation fails
    async fn store_credentials(&self, username: &str, password: &str) -> SecurityResult<()> {
        // Validate inputs before attempting keychain operation
        Validator::validate_username(username)?;
        Validator::validate_password(password)?;

        // Clone for move into blocking task
        let service_name = self.service_name.clone();
        let username = username.to_string();
        let password = password.to_string();

        // CRITICAL: Use spawn_blocking for synchronous keyring operations
        tokio::task::spawn_blocking(move || {
            let entry = Entry::new(&service_name, &username)
                .map_err(|e| SecurityError::KeyringError(e.to_string()))?;

            entry
                .set_password(&password)
                .map_err(|e| SecurityError::KeyringError(e.to_string()))?;

            Ok(())
        })
        .await
        .map_err(|e| SecurityError::InternalError(format!("Task join error: {}", e)))?
    }

    /// Retrieve credentials from the platform keychain
    ///
    /// # Security
    /// - Validates username before retrieval
    /// - Returns password in plaintext (caller must handle securely)
    /// - May prompt user for OS authentication (Touch ID, password, etc.)
    ///
    /// # Errors
    /// - `InvalidInput`: If username validation fails
    /// - `CredentialNotFound`: If no credentials exist for username
    /// - `AccessDenied`: If user denies OS authentication
    /// - `KeyringError`: If platform keychain operation fails
    async fn get_credentials(&self, username: &str) -> SecurityResult<String> {
        // Validate username before attempting retrieval
        Validator::validate_username(username)?;

        // Clone for move into blocking task
        let service_name = self.service_name.clone();
        let username = username.to_string();

        // CRITICAL: Use spawn_blocking for synchronous keyring operations
        tokio::task::spawn_blocking(move || {
            let entry = Entry::new(&service_name, &username)
                .map_err(|e| SecurityError::KeyringError(e.to_string()))?;

            entry.get_password().map_err(|e| match e {
                keyring::Error::NoEntry => SecurityError::CredentialNotFound {
                    username: username.clone(),
                },
                keyring::Error::NoStorageAccess(msg) => SecurityError::AccessDenied(msg.to_string()),
                _ => SecurityError::KeyringError(e.to_string()),
            })
        })
        .await
        .map_err(|e| SecurityError::InternalError(format!("Task join error: {}", e)))?
    }

    /// Delete credentials from the platform keychain
    ///
    /// # Security
    /// - Validates username before deletion
    /// - Irreversible operation (no undo)
    /// - Zero-outs credentials in keychain
    ///
    /// # Errors
    /// - `InvalidInput`: If username validation fails
    /// - `CredentialNotFound`: If no credentials exist for username
    /// - `KeyringError`: If platform keychain operation fails
    async fn delete_credentials(&self, username: &str) -> SecurityResult<()> {
        // Validate username before attempting deletion
        Validator::validate_username(username)?;

        // Clone for move into blocking task
        let service_name = self.service_name.clone();
        let username = username.to_string();

        // CRITICAL: Use spawn_blocking for synchronous keyring operations
        tokio::task::spawn_blocking(move || {
            let entry = Entry::new(&service_name, &username)
                .map_err(|e| SecurityError::KeyringError(e.to_string()))?;

            entry.delete_credential().map_err(|e| match e {
                keyring::Error::NoEntry => SecurityError::CredentialNotFound {
                    username: username.clone(),
                },
                _ => SecurityError::KeyringError(e.to_string()),
            })
        })
        .await
        .map_err(|e| SecurityError::InternalError(format!("Task join error: {}", e)))?
    }

    /// Check if credentials exist for the given username
    ///
    /// # Note
    /// This operation attempts to retrieve the credential to check existence.
    /// It may prompt for OS authentication on some platforms.
    ///
    /// # Errors
    /// - `InvalidInput`: If username validation fails
    /// - `KeyringError`: If platform keychain operation fails
    async fn check_exists(&self, username: &str) -> SecurityResult<bool> {
        // Validate username before attempting check
        Validator::validate_username(username)?;

        // Clone for move into blocking task
        let service_name = self.service_name.clone();
        let username = username.to_string();

        // CRITICAL: Use spawn_blocking for synchronous keyring operations
        tokio::task::spawn_blocking(move || {
            let entry = Entry::new(&service_name, &username)
                .map_err(|e| SecurityError::KeyringError(e.to_string()))?;

            match entry.get_password() {
                Ok(_) => Ok(true),
                Err(keyring::Error::NoEntry) => Ok(false),
                Err(e) => Err(SecurityError::KeyringError(e.to_string())),
            }
        })
        .await
        .map_err(|e| SecurityError::InternalError(format!("Task join error: {}", e)))?
    }

    /// List all account usernames (metadata only, no passwords)
    ///
    /// # Platform Limitations
    /// - The keyring crate does not provide enumeration APIs
    /// - This is a platform limitation, not a RUSTALK limitation
    /// - Returns empty Vec on all platforms
    ///
    /// # Future Enhancement
    /// To support listing accounts, we would need to:
    /// 1. Maintain a separate index in SQLite
    /// 2. Store account metadata separately from credentials
    /// 3. Sync the index with keychain operations
    ///
    /// This is out of scope for Phase 1 (MVP).
    async fn list_accounts(&self) -> SecurityResult<Vec<String>> {
        // The keyring crate does not support enumeration
        // This would require maintaining a separate index
        // For MVP, we return an empty list
        Ok(Vec::new())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Helper to create a test store with a unique service name
    fn create_test_store() -> CredentialStore {
        // Use a unique service name for each test to avoid conflicts
        let service_name = format!("com.rustalk.test.{}", uuid::Uuid::new_v4());
        CredentialStore::with_service_name(service_name)
    }

    #[tokio::test]
    async fn test_store_and_retrieve_credentials() {
        let store = create_test_store();
        let username = "testuser@example.com";
        let password = "test-password-123";

        // Store credentials
        let result = store.store_credentials(username, password).await;
        assert!(result.is_ok(), "Failed to store credentials: {:?}", result.err());

        // Retrieve credentials
        let retrieved = store.get_credentials(username).await;
        assert!(retrieved.is_ok(), "Failed to retrieve credentials: {:?}", retrieved.err());
        assert_eq!(retrieved.unwrap(), password);

        // Cleanup
        let _ = store.delete_credentials(username).await;
    }

    #[tokio::test]
    async fn test_update_credentials() {
        let store = create_test_store();
        let username = "updateuser@example.com";
        let password1 = "first-password";
        let password2 = "second-password";

        // Store initial credentials
        store.store_credentials(username, password1).await.unwrap();

        // Update with new password
        store.store_credentials(username, password2).await.unwrap();

        // Verify new password
        let retrieved = store.get_credentials(username).await.unwrap();
        assert_eq!(retrieved, password2);

        // Cleanup
        let _ = store.delete_credentials(username).await;
    }

    #[tokio::test]
    async fn test_delete_credentials() {
        let store = create_test_store();
        let username = "deleteuser@example.com";
        let password = "password-to-delete";

        // Store credentials
        store.store_credentials(username, password).await.unwrap();

        // Delete credentials
        let result = store.delete_credentials(username).await;
        assert!(result.is_ok(), "Failed to delete credentials: {:?}", result.err());

        // Verify deletion
        let retrieved = store.get_credentials(username).await;
        assert!(retrieved.is_err());
        match retrieved.unwrap_err() {
            SecurityError::CredentialNotFound { .. } => (),
            e => panic!("Expected CredentialNotFound, got: {:?}", e),
        }
    }

    #[tokio::test]
    async fn test_get_nonexistent_credentials() {
        let store = create_test_store();
        let username = "nonexistent@example.com";

        let result = store.get_credentials(username).await;
        assert!(result.is_err());
        match result.unwrap_err() {
            SecurityError::CredentialNotFound { .. } => (),
            e => panic!("Expected CredentialNotFound, got: {:?}", e),
        }
    }

    #[tokio::test]
    async fn test_delete_nonexistent_credentials() {
        let store = create_test_store();
        let username = "nonexistent@example.com";

        let result = store.delete_credentials(username).await;
        assert!(result.is_err());
        match result.unwrap_err() {
            SecurityError::CredentialNotFound { .. } => (),
            e => panic!("Expected CredentialNotFound, got: {:?}", e),
        }
    }

    #[tokio::test]
    async fn test_check_exists_true() {
        let store = create_test_store();
        let username = "existsuser@example.com";
        let password = "exists-password";

        // Store credentials
        store.store_credentials(username, password).await.unwrap();

        // Check exists
        let exists = store.check_exists(username).await.unwrap();
        assert!(exists);

        // Cleanup
        let _ = store.delete_credentials(username).await;
    }

    #[tokio::test]
    async fn test_check_exists_false() {
        let store = create_test_store();
        let username = "notexists@example.com";

        let exists = store.check_exists(username).await.unwrap();
        assert!(!exists);
    }

    #[tokio::test]
    async fn test_invalid_username() {
        let store = create_test_store();
        let invalid_username = "user with spaces";
        let password = "password";

        let result = store.store_credentials(invalid_username, password).await;
        assert!(result.is_err());
        match result.unwrap_err() {
            SecurityError::InvalidInput { field, .. } => {
                assert_eq!(field, "username");
            }
            e => panic!("Expected InvalidInput, got: {:?}", e),
        }
    }

    #[tokio::test]
    async fn test_invalid_password() {
        let store = create_test_store();
        let username = "validuser@example.com";
        let invalid_password = ""; // Empty password

        let result = store.store_credentials(username, invalid_password).await;
        assert!(result.is_err());
        match result.unwrap_err() {
            SecurityError::InvalidInput { field, .. } => {
                assert_eq!(field, "password");
            }
            e => panic!("Expected InvalidInput, got: {:?}", e),
        }
    }

    #[tokio::test]
    async fn test_multiple_accounts() {
        let store = create_test_store();
        let users = vec![
            ("user1@example.com", "password1"),
            ("user2@example.com", "password2"),
            ("user3@example.com", "password3"),
        ];

        // Store all credentials
        for (username, password) in &users {
            store.store_credentials(username, password).await.unwrap();
        }

        // Verify all credentials
        for (username, password) in &users {
            let retrieved = store.get_credentials(username).await.unwrap();
            assert_eq!(&retrieved, password);
        }

        // Cleanup
        for (username, _) in &users {
            let _ = store.delete_credentials(username).await;
        }
    }

    #[tokio::test]
    async fn test_list_accounts_returns_empty() {
        let store = create_test_store();

        // Store some credentials
        store.store_credentials("user1@example.com", "pass1").await.unwrap();
        store.store_credentials("user2@example.com", "pass2").await.unwrap();

        // list_accounts should return empty (not implemented in MVP)
        let accounts = store.list_accounts().await.unwrap();
        assert_eq!(accounts.len(), 0);

        // Cleanup
        let _ = store.delete_credentials("user1@example.com").await;
        let _ = store.delete_credentials("user2@example.com").await;
    }
}
