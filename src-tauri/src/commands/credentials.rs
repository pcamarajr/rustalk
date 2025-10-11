// RUSTALK Credential Management Tauri Commands
//
// This module provides secure credential management commands for SIP accounts.
// All commands include input validation and proper error handling.
//
// Security Features:
// - Input validation before backend calls
// - No password exposure to frontend (read operations)
// - Audit logging for security events
// - Rate limiting placeholders for future implementation

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tauri::State;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{info, warn, error};

// TODO: Replace with actual security module when implemented
// For now, we'll use a placeholder structure
use crate::security::{
    CredentialStore,
    CredentialInfo,
    SecurityResult,
    SecurityError,
};

/// Response structure for credential retrieval (excludes password for security)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CredentialInfoResponse {
    /// SIP username/account identifier
    pub username: String,
    /// SIP server address
    pub server: String,
    /// SIP server port
    pub port: u16,
    /// Transport protocol (UDP, TCP, TLS)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transport: Option<String>,
}

impl From<CredentialInfo> for CredentialInfoResponse {
    fn from(info: CredentialInfo) -> Self {
        Self {
            username: info.username,
            server: info.server,
            port: info.port,
            transport: info.transport,
        }
    }
}

/// Store SIP credentials securely in platform keychain
///
/// # Arguments
/// * `username` - SIP account username
/// * `password` - SIP account password (will be stored securely)
/// * `server` - SIP server address
/// * `port` - SIP server port (default: 5060)
/// * `transport` - Optional transport protocol (UDP, TCP, TLS)
///
/// # TypeScript Signature
/// ```typescript
/// async function storeSipCredentials(
///   username: string,
///   password: string,
///   server: string,
///   port: number,
///   transport?: string
/// ): Promise<void>
/// ```
///
/// # Security
/// - Validates all input parameters before storage
/// - Maximum length constraints enforced
/// - Character whitelisting for security
/// - Stores password in platform keychain (Keychain/Credential Manager)
///
/// # Errors
/// - Returns error string if validation fails
/// - Returns error if credential already exists
/// - Returns error if platform keychain access fails
#[tauri::command]
pub async fn store_sip_credentials(
    state: State<'_, Arc<RwLock<CredentialStore>>>,
    username: String,
    password: String,
    server: String,
    port: u16,
    transport: Option<String>,
) -> Result<(), String> {
    info!(
        "Storing SIP credentials for user: {} (server: {}:{})",
        username, server, port
    );

    // Input validation
    validate_username(&username)?;
    validate_password(&password)?;
    validate_server(&server)?;
    validate_port(port)?;

    if let Some(ref t) = transport {
        validate_transport(t)?;
    }

    // Acquire write lock on credential store
    let store = state.write().await;

    // Store credentials (backend handles keychain interaction)
    match store.store_credential(&username, &password, &server, port, transport.as_deref()).await {
        Ok(()) => {
            info!("Successfully stored credentials for user: {}", username);
            // TODO: Add audit log entry
            Ok(())
        }
        Err(e) => {
            error!("Failed to store credentials for user {}: {:?}", username, e);
            Err(convert_security_error(e))
        }
    }
}

/// Retrieve SIP credentials (without password)
///
/// # Arguments
/// * `username` - SIP account username to retrieve
///
/// # TypeScript Signature
/// ```typescript
/// async function getSipCredentials(
///   username: string
/// ): Promise<CredentialInfoResponse>
/// ```
///
/// # Security
/// - Password is NEVER returned to frontend for security
/// - Only returns non-sensitive account information
/// - Validates username before retrieval
///
/// # Errors
/// - Returns error if credential not found
/// - Returns error if platform keychain access fails
#[tauri::command]
pub async fn get_sip_credentials(
    state: State<'_, Arc<RwLock<CredentialStore>>>,
    username: String,
) -> Result<CredentialInfoResponse, String> {
    info!("Retrieving SIP credentials for user: {}", username);

    // Input validation
    validate_username(&username)?;

    // Acquire read lock on credential store
    let store = state.read().await;

    // Retrieve credential info (without password)
    match store.get_credential_info(&username).await {
        Ok(info) => {
            info!("Successfully retrieved credentials for user: {}", username);
            // TODO: Add audit log entry
            Ok(info.into())
        }
        Err(e) => {
            warn!("Failed to retrieve credentials for user {}: {:?}", username, e);
            Err(convert_security_error(e))
        }
    }
}

/// Delete SIP credentials from keychain
///
/// # Arguments
/// * `username` - SIP account username to delete
///
/// # TypeScript Signature
/// ```typescript
/// async function deleteSipCredentials(
///   username: string
/// ): Promise<void>
/// ```
///
/// # Security
/// - Validates username before deletion
/// - Permanently removes from platform keychain
/// - Logs deletion for audit trail
///
/// # Errors
/// - Returns error if credential not found
/// - Returns error if platform keychain access fails
#[tauri::command]
pub async fn delete_sip_credentials(
    state: State<'_, Arc<RwLock<CredentialStore>>>,
    username: String,
) -> Result<(), String> {
    info!("Deleting SIP credentials for user: {}", username);

    // Input validation
    validate_username(&username)?;

    // Acquire write lock on credential store
    let store = state.write().await;

    // Delete credential
    match store.delete_credential(&username).await {
        Ok(()) => {
            info!("Successfully deleted credentials for user: {}", username);
            // TODO: Add audit log entry
            Ok(())
        }
        Err(e) => {
            error!("Failed to delete credentials for user {}: {:?}", username, e);
            Err(convert_security_error(e))
        }
    }
}

/// Check if SIP credentials exist for a username
///
/// # Arguments
/// * `username` - SIP account username to check
///
/// # TypeScript Signature
/// ```typescript
/// async function checkCredentialExists(
///   username: string
/// ): Promise<boolean>
/// ```
///
/// # Security
/// - Validates username before checking
/// - Does not reveal credential details
///
/// # Errors
/// - Returns error if validation fails
/// - Returns false if credential doesn't exist (not an error)
#[tauri::command]
pub async fn check_credential_exists(
    state: State<'_, Arc<RwLock<CredentialStore>>>,
    username: String,
) -> Result<bool, String> {
    // Input validation
    validate_username(&username)?;

    // Acquire read lock on credential store
    let store = state.read().await;

    // Check existence
    match store.credential_exists(&username).await {
        Ok(exists) => Ok(exists),
        Err(e) => {
            warn!("Failed to check credential existence for user {}: {:?}", username, e);
            Err(convert_security_error(e))
        }
    }
}

/// List all stored SIP account usernames
///
/// # TypeScript Signature
/// ```typescript
/// async function listSipAccounts(): Promise<string[]>
/// ```
///
/// # Security
/// - Only returns usernames, no sensitive data
/// - May return empty list on some platforms (macOS limitation)
///
/// # Platform Notes
/// - Windows: Full enumeration supported
/// - macOS: Keychain doesn't support enumeration, returns empty list
///
/// # Errors
/// - Returns error if platform keychain access fails
#[tauri::command]
pub async fn list_sip_accounts(
    state: State<'_, Arc<RwLock<CredentialStore>>>,
) -> Result<Vec<String>, String> {
    info!("Listing all SIP accounts");

    // Acquire read lock on credential store
    let store = state.read().await;

    // List accounts
    match store.list_accounts().await {
        Ok(accounts) => {
            info!("Found {} SIP accounts", accounts.len());
            Ok(accounts)
        }
        Err(e) => {
            // macOS may not support enumeration - return empty list instead of error
            if matches!(e, SecurityError::PlatformNotSupported(_)) {
                warn!("Platform does not support credential enumeration (likely macOS)");
                Ok(Vec::new())
            } else {
                error!("Failed to list SIP accounts: {:?}", e);
                Err(convert_security_error(e))
            }
        }
    }
}

// ============================================================================
// Input Validation Functions
// ============================================================================

/// Maximum field lengths for security
const MAX_USERNAME_LENGTH: usize = 256;
const MAX_PASSWORD_LENGTH: usize = 4096;
const MAX_SERVER_LENGTH: usize = 256;
const MAX_TRANSPORT_LENGTH: usize = 16;

/// Validate username format and length
fn validate_username(username: &str) -> Result<(), String> {
    if username.is_empty() {
        return Err("Username cannot be empty".to_string());
    }

    if username.len() > MAX_USERNAME_LENGTH {
        return Err(format!(
            "Username exceeds maximum length of {} characters",
            MAX_USERNAME_LENGTH
        ));
    }

    // Allow alphanumeric, dots, underscores, percent, plus, at, and dash
    // This covers most SIP username formats
    if !username
        .chars()
        .all(|c| c.is_alphanumeric() || "._%+@-".contains(c))
    {
        return Err("Username contains invalid characters. Allowed: alphanumeric, . _ % + @ -".to_string());
    }

    Ok(())
}

/// Validate password format and length
fn validate_password(password: &str) -> Result<(), String> {
    if password.is_empty() {
        return Err("Password cannot be empty".to_string());
    }

    if password.len() > MAX_PASSWORD_LENGTH {
        return Err(format!(
            "Password exceeds maximum length of {} characters",
            MAX_PASSWORD_LENGTH
        ));
    }

    // Check for null bytes (can cause issues in C FFI)
    if password.contains('\0') {
        return Err("Password contains invalid null bytes".to_string());
    }

    Ok(())
}

/// Validate server address format and length
fn validate_server(server: &str) -> Result<(), String> {
    if server.is_empty() {
        return Err("Server address cannot be empty".to_string());
    }

    if server.len() > MAX_SERVER_LENGTH {
        return Err(format!(
            "Server address exceeds maximum length of {} characters",
            MAX_SERVER_LENGTH
        ));
    }

    // Basic hostname/IP validation (allow alphanumeric, dots, dash, colon for IPv6)
    if !server
        .chars()
        .all(|c| c.is_alphanumeric() || ".-:".contains(c))
    {
        return Err("Server address contains invalid characters".to_string());
    }

    Ok(())
}

/// Validate port number
fn validate_port(port: u16) -> Result<(), String> {
    // Port 0 is invalid
    if port == 0 {
        return Err("Port cannot be 0".to_string());
    }

    // Warn about privileged ports (< 1024) but allow them
    if port < 1024 {
        warn!("Using privileged port {} - may require elevated permissions", port);
    }

    Ok(())
}

/// Validate transport protocol
fn validate_transport(transport: &str) -> Result<(), String> {
    if transport.is_empty() {
        return Err("Transport cannot be empty".to_string());
    }

    if transport.len() > MAX_TRANSPORT_LENGTH {
        return Err(format!(
            "Transport exceeds maximum length of {} characters",
            MAX_TRANSPORT_LENGTH
        ));
    }

    // Only allow specific transport types
    let transport_lower = transport.to_lowercase();
    match transport_lower.as_str() {
        "udp" | "tcp" | "tls" | "ws" | "wss" => Ok(()),
        _ => Err(format!(
            "Invalid transport '{}'. Allowed: UDP, TCP, TLS, WS, WSS",
            transport
        )),
    }
}

// ============================================================================
// Error Conversion
// ============================================================================

/// Convert SecurityError to user-friendly error string
fn convert_security_error(error: SecurityError) -> String {
    match error {
        SecurityError::CredentialNotFound { username } => {
            format!("Credentials not found for user: {}", username)
        }
        SecurityError::CredentialAlreadyExists { username } => {
            format!("Credentials already exist for user: {}", username)
        }
        SecurityError::InvalidInput { field, reason } => {
            format!("Invalid {}: {}", field, reason)
        }
        SecurityError::PlatformError(msg) => {
            format!("Platform error: {}", msg)
        }
        SecurityError::PlatformNotSupported(feature) => {
            format!("Platform does not support: {}", feature)
        }
        SecurityError::AccessDenied(msg) => {
            format!("Access denied: {}", msg)
        }
        SecurityError::DataCorruption(msg) => {
            format!("Data corruption detected: {}", msg)
        }
        SecurityError::IoError(e) => {
            format!("I/O error: {}", e)
        }
        SecurityError::SerializationError(e) => {
            format!("Serialization error: {}", e)
        }
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_username_valid() {
        assert!(validate_username("user@example.com").is_ok());
        assert!(validate_username("user123").is_ok());
        assert!(validate_username("user.name").is_ok());
        assert!(validate_username("user_name").is_ok());
        assert!(validate_username("user-name").is_ok());
        assert!(validate_username("user+name").is_ok());
        assert!(validate_username("user%name").is_ok());
    }

    #[test]
    fn test_validate_username_invalid() {
        assert!(validate_username("").is_err());
        assert!(validate_username(&"a".repeat(300)).is_err());
        assert!(validate_username("user name").is_err()); // space
        assert!(validate_username("user/name").is_err()); // slash
        assert!(validate_username("user\\name").is_err()); // backslash
    }

    #[test]
    fn test_validate_password_valid() {
        assert!(validate_password("password123").is_ok());
        assert!(validate_password("p@ssw0rd!").is_ok());
        assert!(validate_password(&"a".repeat(100)).is_ok());
    }

    #[test]
    fn test_validate_password_invalid() {
        assert!(validate_password("").is_err());
        assert!(validate_password(&"a".repeat(5000)).is_err());
        assert!(validate_password("pass\0word").is_err()); // null byte
    }

    #[test]
    fn test_validate_server_valid() {
        assert!(validate_server("sip.example.com").is_ok());
        assert!(validate_server("192.168.1.1").is_ok());
        assert!(validate_server("example-server.com").is_ok());
        assert!(validate_server("2001:db8::1").is_ok()); // IPv6
    }

    #[test]
    fn test_validate_server_invalid() {
        assert!(validate_server("").is_err());
        assert!(validate_server(&"a".repeat(300)).is_err());
        assert!(validate_server("server name").is_err()); // space
        assert!(validate_server("server/path").is_err()); // slash
    }

    #[test]
    fn test_validate_port_valid() {
        assert!(validate_port(5060).is_ok());
        assert!(validate_port(5061).is_ok());
        assert!(validate_port(65535).is_ok());
        assert!(validate_port(80).is_ok()); // privileged but allowed
    }

    #[test]
    fn test_validate_port_invalid() {
        assert!(validate_port(0).is_err());
    }

    #[test]
    fn test_validate_transport_valid() {
        assert!(validate_transport("UDP").is_ok());
        assert!(validate_transport("TCP").is_ok());
        assert!(validate_transport("TLS").is_ok());
        assert!(validate_transport("WS").is_ok());
        assert!(validate_transport("WSS").is_ok());
        assert!(validate_transport("udp").is_ok()); // case insensitive
        assert!(validate_transport("tcp").is_ok());
    }

    #[test]
    fn test_validate_transport_invalid() {
        assert!(validate_transport("").is_err());
        assert!(validate_transport("INVALID").is_err());
        assert!(validate_transport("HTTP").is_err());
        assert!(validate_transport(&"a".repeat(20)).is_err());
    }
}
