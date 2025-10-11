// Credential service implementation
//
// Main API for credential management, orchestrating keychain and database operations.

use crate::security::{
    database::AccountDatabase,
    error::CredentialError,
    keychain_adapter::KeychainAdapter,
    types::{AccountUpdate, CredentialService, MockMode, SipAccount, SipCredentials},
    Validator,
};

impl CredentialService {
    /// Create new production credential service
    pub async fn new() -> Result<Self, CredentialError> {
        // Verify platform support
        #[cfg(not(any(target_os = "macos", target_os = "windows", target_os = "linux")))]
        {
            return Err(CredentialError::Validation(
                "Unsupported platform for credential storage".to_string(),
            ));
        }

        Ok(Self {
            keychain: KeychainAdapter::new("com.rustalk.sip"),
            database: AccountDatabase::new(),
            mock_mode: None,
        })
    }

    /// Create test service with mock keychain (for unit tests)
    pub async fn new_test() -> Result<Self, CredentialError> {
        Ok(Self {
            keychain: KeychainAdapter::new("com.rustalk.test"),
            database: AccountDatabase::new(),
            mock_mode: Some(MockMode::Normal),
        })
    }

    /// Create mock service that simulates unavailable keychain
    pub async fn new_mock_unavailable() -> Result<Self, CredentialError> {
        Err(CredentialError::Keychain(
            crate::security::error::KeychainError::Unavailable,
        ))
    }

    /// Create mock service that simulates permission denied
    pub async fn new_mock_permission_denied() -> Result<Self, CredentialError> {
        Err(CredentialError::Keychain(
            crate::security::error::KeychainError::PermissionDenied,
        ))
    }

    /// Store SIP account with credentials
    pub async fn store_credentials(
        &self,
        account: SipAccount,
        credentials: SipCredentials,
    ) -> Result<String, CredentialError> {
        // Validate account data
        Validator::validate_server_host(&account.server_host)?;
        Validator::validate_port(account.server_port)?;

        // Store credentials in keychain (using account ID as keychain key)
        self.keychain
            .store(&account.id, &credentials.password)
            .await?;

        // Store account metadata in database
        self.database.insert(account.clone()).await?;

        Ok(account.id)
    }

    /// Retrieve SIP account and credentials
    pub async fn get_credentials(
        &self,
        account_id: &str,
    ) -> Result<(SipAccount, SipCredentials), CredentialError> {
        // Get account metadata
        let account = self.database.get(account_id).await?;

        // Get password from keychain
        let password = self.keychain.get(account_id).await?;

        // Create credentials (username can be derived from account or stored separately)
        // For now, we'll use a simple approach where username is in the account
        // In production, this would be part of the account structure
        let credentials = SipCredentials {
            username: format!("{}@{}", account_id, account.server_host), // Temporary
            password,
        };

        Ok((account, credentials))
    }

    /// Update account credentials
    pub async fn update_credentials(
        &self,
        account_id: &str,
        credentials: SipCredentials,
    ) -> Result<(), CredentialError> {
        // Verify account exists
        self.database.get(account_id).await?;

        // Update credentials in keychain
        self.keychain
            .store(account_id, &credentials.password)
            .await?;

        Ok(())
    }

    /// Update account metadata
    pub async fn update_account(
        &self,
        account_id: &str,
        updates: AccountUpdate,
    ) -> Result<(), CredentialError> {
        // Validate updates
        if let Some(ref host) = updates.server_host {
            Validator::validate_server_host(host)?;
        }

        if let Some(port) = updates.server_port {
            Validator::validate_port(port)?;
        }

        self.database.update(account_id, updates).await
    }

    /// Delete account and credentials
    pub async fn delete_account(&self, account_id: &str) -> Result<(), CredentialError> {
        // Verify account exists
        self.database.get(account_id).await?;

        // Delete from keychain
        self.keychain.delete(account_id).await?;

        // Delete from database
        self.database.delete(account_id).await?;

        Ok(())
    }

    /// List all accounts (without passwords)
    pub async fn list_accounts(&self) -> Result<Vec<SipAccount>, CredentialError> {
        self.database.list().await
    }

    /// Check if account exists
    pub async fn check_exists(&self, account_id: &str) -> Result<bool, CredentialError> {
        match self.database.get(account_id).await {
            Ok(_) => Ok(true),
            Err(CredentialError::NotFound) => Ok(false),
            Err(e) => Err(e),
        }
    }
}
