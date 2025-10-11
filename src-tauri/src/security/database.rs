// In-memory account database
//
// For MVP, we use an in-memory HashMap to store account metadata.
// In production, this would be replaced with SQLite or similar.

use crate::security::{
    error::CredentialError,
    types::{AccountUpdate, SipAccount},
};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

/// In-memory database for account metadata
pub struct AccountDatabase {
    accounts: Arc<RwLock<HashMap<String, SipAccount>>>,
}

impl AccountDatabase {
    /// Create new account database
    pub fn new() -> Self {
        Self {
            accounts: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Insert new account
    pub async fn insert(&self, account: SipAccount) -> Result<(), CredentialError> {
        let mut accounts = self.accounts.write().await;

        if accounts.contains_key(&account.id) {
            return Err(CredentialError::AlreadyExists);
        }

        accounts.insert(account.id.clone(), account);
        Ok(())
    }

    /// Get account by ID
    pub async fn get(&self, account_id: &str) -> Result<SipAccount, CredentialError> {
        let accounts = self.accounts.read().await;
        accounts
            .get(account_id)
            .cloned()
            .ok_or(CredentialError::NotFound)
    }

    /// Update account metadata
    pub async fn update(
        &self,
        account_id: &str,
        updates: AccountUpdate,
    ) -> Result<(), CredentialError> {
        let mut accounts = self.accounts.write().await;

        let account = accounts
            .get_mut(account_id)
            .ok_or(CredentialError::NotFound)?;

        if let Some(display_name) = updates.display_name {
            account.display_name = display_name;
        }

        if let Some(server_host) = updates.server_host {
            account.server_host = server_host;
        }

        if let Some(server_port) = updates.server_port {
            account.server_port = server_port;
        }

        if let Some(status) = updates.status {
            account.status = status;
        }

        account.updated_at = chrono::Utc::now();

        Ok(())
    }

    /// Delete account
    pub async fn delete(&self, account_id: &str) -> Result<(), CredentialError> {
        let mut accounts = self.accounts.write().await;
        accounts
            .remove(account_id)
            .ok_or(CredentialError::NotFound)?;
        Ok(())
    }

    /// List all accounts
    pub async fn list(&self) -> Result<Vec<SipAccount>, CredentialError> {
        let accounts = self.accounts.read().await;
        Ok(accounts.values().cloned().collect())
    }
}

impl Default for AccountDatabase {
    fn default() -> Self {
        Self::new()
    }
}
