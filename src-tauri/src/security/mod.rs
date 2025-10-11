// Security module - Secure credential storage for RUSTALK
//
// This module provides platform-native credential storage using:
// - macOS: Keychain Services
// - Windows: Credential Manager
//
// All keyring operations are wrapped in tokio::task::spawn_blocking
// to avoid blocking the async runtime (keyring is synchronous).

mod database;
mod error;
mod errors;
mod keychain;
mod keychain_adapter;
mod service;
mod types;
mod validation;

#[cfg(test)]
mod tests;

// Export error types
pub use error::{CredentialError, KeychainError, SecurityError, SecurityResult};

// Export types
pub use types::{
    AccountStatus, AccountUpdate, Credential, CredentialInfo, CredentialService, MockMode,
    SecureCredential, SipAccount, SipCredentials, SipTransport,
};

// Export keychain provider (legacy)
pub use keychain::{CredentialStore, KeychainProvider};

// Export database (internal use)
pub(crate) use database::AccountDatabase;

// Export keychain adapter (internal use)
pub(crate) use keychain_adapter::KeychainAdapter;

// Export validator
pub use validation::Validator;

/// Service name for RUSTALK SIP credentials
pub const SIP_SERVICE_NAME: &str = "com.rustalk.sip";
