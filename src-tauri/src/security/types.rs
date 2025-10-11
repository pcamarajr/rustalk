// RUSTALK Security Types
//
// Common types used throughout the security module.

use crate::security::error::{CredentialError, KeychainError};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// SIP transport protocol
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum SipTransport {
    /// UDP transport
    Udp,
    /// TCP transport
    Tcp,
    /// TLS transport (SIPS)
    Sips,
}

impl Default for SipTransport {
    fn default() -> Self {
        SipTransport::Sips // Secure by default
    }
}

/// Account status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum AccountStatus {
    /// Account is active
    Active,
    /// Account is disabled
    Disabled,
    /// Account is pending verification
    Pending,
}

impl Default for AccountStatus {
    fn default() -> Self {
        AccountStatus::Active
    }
}

/// SIP account metadata (without credentials)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SipAccount {
    /// Unique account identifier
    pub id: String,
    /// Display name for UI
    pub display_name: String,
    /// SIP server hostname
    pub server_host: String,
    /// SIP server port
    pub server_port: u16,
    /// Transport protocol
    pub transport: SipTransport,
    /// Account status
    pub status: AccountStatus,
    /// Creation timestamp
    pub created_at: DateTime<Utc>,
    /// Last update timestamp
    pub updated_at: DateTime<Utc>,
}

/// SIP credentials (username + password)
#[derive(Debug, Clone)]
pub struct SipCredentials {
    /// SIP username
    pub username: String,
    /// SIP password (should be zeroized on drop in production)
    pub password: String,
}

impl SipCredentials {
    /// Create new SIP credentials with validation
    pub fn new(username: String, password: String) -> Result<Self, CredentialError> {
        // Validate username
        crate::security::Validator::validate_username(&username)?;

        // Validate password
        crate::security::Validator::validate_password(&password)?;

        Ok(Self { username, password })
    }
}

// Implement Debug to redact password
impl std::fmt::Debug for SipCredentials {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SipCredentials")
            .field("username", &self.username)
            .field("password", &"<redacted>")
            .finish()
    }
}

/// Account update struct (all fields optional)
#[derive(Debug, Clone, Default)]
pub struct AccountUpdate {
    /// New display name
    pub display_name: Option<String>,
    /// New server host
    pub server_host: Option<String>,
    /// New server port
    pub server_port: Option<u16>,
    /// New status
    pub status: Option<AccountStatus>,
}

/// Credential service (main API)
pub struct CredentialService {
    pub(crate) keychain: crate::security::keychain::KeychainAdapter,
    pub(crate) database: crate::security::database::AccountDatabase,
    pub(crate) mock_mode: Option<MockMode>,
}

/// Mock modes for testing
#[derive(Debug, Clone)]
pub enum MockMode {
    Normal,
    Unavailable,
    PermissionDenied,
}

// Legacy types for backward compatibility
/// Credential storage record (without secret)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Credential {
    /// Service identifier (e.g., "rustalk.sip")
    pub service: String,
    /// Account/username
    pub account: String,
    /// Additional attributes (server, port, transport, etc.)
    #[serde(default)]
    pub attributes: HashMap<String, String>,
}

/// Secure credential data (includes secret)
#[derive(Debug, Clone)]
pub struct SecureCredential {
    /// Credential metadata
    pub credential: Credential,
    /// Secret value (password, token, etc.)
    /// This field is intentionally not serialized to prevent logging
    pub secret: String,
}

/// Credential information for frontend (excludes password)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CredentialInfo {
    /// SIP username
    pub username: String,
    /// SIP server address
    pub server: String,
    /// SIP server port
    pub port: u16,
    /// Transport protocol (UDP, TCP, TLS)
    pub transport: Option<String>,
}
