# RUSTALK Phase 1: Security & Storage Specification

**Status:** Draft
**Version:** 1.0.0
**Last Updated:** 2025-10-11
**Owner:** Security Team

---

## 1. Feature Overview

### 1.1 Purpose

The Security & Storage feature provides secure credential management for RUSTALK's SIP accounts. This is a **critical MVP blocker** that enables users to safely store and retrieve SIP authentication credentials while ensuring compliance with platform security best practices.

### 1.2 Scope

**In Scope:**
- Secure storage of SIP credentials using platform keychain APIs
- CRUD operations for multiple SIP accounts
- TLS/SIPS connection enforcement
- Certificate validation for SIP servers
- Input validation and sanitization
- Secure memory handling for sensitive data

**Out of Scope (Post-MVP):**
- Biometric authentication (Touch ID, Windows Hello)
- Credential import/export
- Cloud sync of credentials
- Password strength meter
- Two-factor authentication
- Certificate pinning

### 1.3 Success Metrics

- **Security:** 0 plaintext credential storage
- **Reliability:** 99.9% keychain operation success rate
- **Coverage:** 85%+ test coverage for security-critical code
- **Performance:** <50ms for credential retrieval
- **Audit:** Pass automated security scanning (cargo audit)

### 1.4 Dependencies

- **Rust Crates:**
  - `keyring` (v2.x) - Cross-platform keychain access
  - `rustls` (v0.21.x) - TLS implementation
  - `webpki-roots` - Certificate validation
  - `zeroize` (v1.x) - Secure memory clearing

- **Platform APIs:**
  - macOS: Security Framework (Keychain Services)
  - Windows: Credential Manager API

---

## 2. Functional Requirements

### FR-2.1: Credential Storage

**FR-2.1.1: Store SIP Credentials**
- **Priority:** Critical
- **Description:** System shall securely store SIP account credentials in platform keychain
- **Acceptance Criteria:**
  - ✅ Credentials stored using platform-native keychain API
  - ✅ Each account identified by unique service identifier
  - ✅ No plaintext storage in files or memory dumps
  - ✅ Credentials encrypted by OS keychain mechanism
  - ✅ Application requires OS-level authentication to access

**FR-2.1.2: Support Multiple Accounts**
- **Priority:** High
- **Description:** System shall support storing multiple SIP accounts simultaneously
- **Acceptance Criteria:**
  - ✅ Each account has unique identifier (e.g., `rustalk.sip.account_001`)
  - ✅ Account metadata stored separately (display name, server)
  - ✅ No cross-contamination between account credentials
  - ✅ Support minimum 10 accounts per installation

**FR-2.1.3: Credential Attributes**
- **Priority:** Critical
- **Description:** System shall store all required SIP authentication attributes
- **Acceptance Criteria:**
  - ✅ Username (SIP URI or auth username)
  - ✅ Password (stored securely, never logged)
  - ✅ SIP server hostname/IP
  - ✅ SIP server port (default: 5061 for SIPS)
  - ✅ Transport protocol (SIPS/TLS required)
  - ✅ Display name (optional, for UI)
  - ✅ Created/modified timestamps

### FR-2.2: Credential Retrieval

**FR-2.2.1: Retrieve Credentials**
- **Priority:** Critical
- **Description:** System shall retrieve stored credentials for SIP registration
- **Acceptance Criteria:**
  - ✅ Retrieve by account identifier
  - ✅ Return decrypted credentials in memory-safe structure
  - ✅ Fail gracefully if keychain unavailable
  - ✅ Complete within 50ms (p95)
  - ✅ Zero-out credentials after use

**FR-2.2.2: List Accounts**
- **Priority:** High
- **Description:** System shall list all stored SIP accounts (metadata only)
- **Acceptance Criteria:**
  - ✅ Return account IDs and display names
  - ✅ Never return passwords in list operation
  - ✅ Sort by creation date (newest first)
  - ✅ Include account status (active/inactive)

### FR-2.3: Credential Modification

**FR-2.3.1: Update Credentials**
- **Priority:** High
- **Description:** System shall allow updating existing account credentials
- **Acceptance Criteria:**
  - ✅ Update password without re-entering other fields
  - ✅ Update server settings independently
  - ✅ Maintain account identifier on update
  - ✅ Update modified timestamp
  - ✅ Validate new values before storing

**FR-2.3.2: Delete Credentials**
- **Priority:** Critical
- **Description:** System shall securely delete account credentials
- **Acceptance Criteria:**
  - ✅ Remove from platform keychain
  - ✅ Zero-out in-memory copies
  - ✅ Delete associated metadata
  - ✅ Confirm deletion to user
  - ✅ Irreversible operation (no undo)

### FR-2.4: Security Enforcement

**FR-2.4.1: TLS/SIPS Only**
- **Priority:** Critical
- **Description:** System shall enforce encrypted SIP connections
- **Acceptance Criteria:**
  - ✅ Reject non-TLS SIP connections
  - ✅ Require SIPS (SIP over TLS, port 5061)
  - ✅ Display security warning for non-standard ports
  - ✅ Use TLS 1.2+ only (no SSLv3, TLS 1.0, TLS 1.1)

**FR-2.4.2: Certificate Validation**
- **Priority:** Critical
- **Description:** System shall validate SIP server certificates
- **Acceptance Criteria:**
  - ✅ Verify certificate chain against system trust store
  - ✅ Check certificate expiration
  - ✅ Validate hostname matches certificate CN/SAN
  - ✅ Reject self-signed certificates by default
  - ✅ Log validation failures with reason

**FR-2.4.3: Input Validation**
- **Priority:** Critical
- **Description:** System shall validate all user inputs for security
- **Acceptance Criteria:**
  - ✅ Sanitize SIP server hostnames (prevent injection)
  - ✅ Validate port ranges (1-65535)
  - ✅ Limit string lengths (username: 256, password: 256)
  - ✅ Reject control characters in text fields
  - ✅ Return descriptive validation errors

**FR-2.4.4: Secure Memory Handling**
- **Priority:** High
- **Description:** System shall prevent sensitive data leaks in memory
- **Acceptance Criteria:**
  - ✅ Use `zeroize` crate for password fields
  - ✅ Zero-out credentials after use
  - ✅ Never log passwords or tokens
  - ✅ Prevent passwords in panic messages
  - ✅ Clear credentials from memory on logout

---

## 3. Non-Functional Requirements

### NFR-3.1: Performance

**NFR-3.1.1: Credential Retrieval Latency**
- **Target:** <50ms for p95 latency
- **Measurement:** Rust benchmark tests
- **Validation:** Performance regression tests in CI

**NFR-3.1.2: Keychain Operation Throughput**
- **Target:** Support 5+ concurrent credential operations
- **Measurement:** Tokio async task monitoring
- **Validation:** Load testing during E2E tests

### NFR-3.2: Security

**NFR-3.2.1: OWASP Compliance**
- **Requirement:** Address OWASP Top 10 relevant issues
- **Focus Areas:**
  - A02:2021 – Cryptographic Failures (credential storage)
  - A03:2021 – Injection (input validation)
  - A07:2021 – Identification and Authentication Failures
- **Validation:** Manual security review + automated scanning

**NFR-3.2.2: Secure Defaults**
- **Requirement:** All security features enabled by default
- **Examples:**
  - TLS 1.2+ required
  - Certificate validation enabled
  - Secure memory handling active
- **Validation:** Configuration audit in CI

**NFR-3.2.3: Audit Logging**
- **Requirement:** Log all security-relevant events
- **Events:**
  - Credential access attempts
  - Keychain operation failures
  - TLS handshake failures
  - Certificate validation errors
- **Format:** Structured logs (JSON) with timestamps
- **Storage:** Local file, never sent to network

### NFR-3.3: Reliability

**NFR-3.3.1: Keychain Availability**
- **Target:** 99.9% success rate for keychain operations
- **Handling:** Graceful degradation if keychain unavailable
- **Validation:** Chaos testing with keychain failures

**NFR-3.3.2: Error Recovery**
- **Requirement:** System continues operation after transient errors
- **Examples:**
  - Retry keychain operations (3 attempts, exponential backoff)
  - Prompt user for OS authentication if required
  - Cache credentials in secure memory (session only)

### NFR-3.4: Usability

**NFR-3.4.1: Error Messages**
- **Requirement:** Clear, actionable error messages
- **Examples:**
  - "Keychain access denied. Grant RUSTALK permission in System Preferences."
  - "SIP server certificate expired. Contact your provider."
- **Validation:** UX review of all error scenarios

**NFR-3.4.2: Platform Integration**
- **Requirement:** Native OS dialogs for keychain access
- **macOS:** Use Security Framework prompts
- **Windows:** Use Credential Manager UI
- **Validation:** Manual testing on each platform

---

## 4. Data Model

### 4.1 SIP Account Entity

```rust
/// SIP account stored in keychain and local database
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SipAccount {
    /// Unique identifier (UUID v4)
    pub id: String,

    /// Display name for UI (e.g., "Work Phone")
    pub display_name: String,

    /// SIP server hostname or IP
    pub server_host: String,

    /// SIP server port (default: 5061)
    pub server_port: u16,

    /// Transport protocol (must be "SIPS")
    pub transport: SipTransport,

    /// Account status
    pub status: AccountStatus,

    /// Creation timestamp (UTC)
    pub created_at: DateTime<Utc>,

    /// Last modified timestamp (UTC)
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SipTransport {
    /// SIPS (SIP over TLS) - REQUIRED
    Sips,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AccountStatus {
    /// Account active and ready for use
    Active,

    /// Account disabled by user
    Disabled,

    /// Account configuration error
    ConfigError(String),
}
```

### 4.2 Credential Storage Format

**Keychain Entry:**
- **Service:** `com.rustalk.sip.{account_id}`
- **Account:** `{username}`
- **Password:** `{password}` (encrypted by OS)

**Metadata Storage (SQLite):**
```sql
CREATE TABLE sip_accounts (
    id TEXT PRIMARY KEY,
    display_name TEXT NOT NULL,
    server_host TEXT NOT NULL,
    server_port INTEGER NOT NULL,
    transport TEXT NOT NULL DEFAULT 'SIPS',
    status TEXT NOT NULL DEFAULT 'Active',
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE INDEX idx_accounts_created ON sip_accounts(created_at DESC);
```

### 4.3 Secure Credential Struct

```rust
use zeroize::{Zeroize, ZeroizeOnDrop};

/// Secure credential container (zero-out on drop)
#[derive(Zeroize, ZeroizeOnDrop)]
pub struct SipCredentials {
    /// SIP authentication username
    #[zeroize(skip)]
    pub username: String,

    /// SIP authentication password (NEVER log this)
    pub password: String,
}

impl SipCredentials {
    /// Create new credentials (validate inputs)
    pub fn new(username: String, password: String) -> Result<Self, ValidationError> {
        validate_username(&username)?;
        validate_password(&password)?;
        Ok(Self { username, password })
    }
}

// Prevent accidental logging of passwords
impl std::fmt::Debug for SipCredentials {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SipCredentials")
            .field("username", &self.username)
            .field("password", &"<redacted>")
            .finish()
    }
}
```

---

## 5. API Specification

### 5.1 Rust API

```rust
/// Credential storage service
pub struct CredentialService {
    keyring: Box<dyn KeyringBackend>,
    metadata_db: SqlitePool,
}

impl CredentialService {
    /// Store new SIP account credentials
    ///
    /// # Security
    /// - Validates all inputs before storage
    /// - Stores password in platform keychain
    /// - Metadata stored in local SQLite (no passwords)
    ///
    /// # Errors
    /// - `KeychainError` if platform keychain unavailable
    /// - `ValidationError` if inputs invalid
    /// - `DatabaseError` if metadata storage fails
    pub async fn store_credentials(
        &self,
        account: SipAccount,
        credentials: SipCredentials,
    ) -> Result<String, CredentialError>;

    /// Retrieve credentials for account
    ///
    /// # Security
    /// - Requires OS authentication (keychain prompt)
    /// - Returns secure struct that zero-outs on drop
    ///
    /// # Errors
    /// - `NotFound` if account doesn't exist
    /// - `KeychainError` if OS authentication fails
    pub async fn get_credentials(
        &self,
        account_id: &str,
    ) -> Result<(SipAccount, SipCredentials), CredentialError>;

    /// Update existing credentials
    ///
    /// # Security
    /// - Validates new values before update
    /// - Atomic operation (all or nothing)
    pub async fn update_credentials(
        &self,
        account_id: &str,
        credentials: SipCredentials,
    ) -> Result<(), CredentialError>;

    /// Update account metadata only (no password)
    pub async fn update_account(
        &self,
        account_id: &str,
        updates: AccountUpdate,
    ) -> Result<(), CredentialError>;

    /// Delete account and credentials
    ///
    /// # Security
    /// - Removes from keychain and database
    /// - Zero-outs in-memory copies
    /// - Irreversible operation
    pub async fn delete_account(
        &self,
        account_id: &str,
    ) -> Result<(), CredentialError>;

    /// List all accounts (metadata only, no passwords)
    pub async fn list_accounts(&self) -> Result<Vec<SipAccount>, CredentialError>;

    /// Validate account can connect (check TLS/certificate)
    pub async fn validate_account(
        &self,
        account_id: &str,
    ) -> Result<ValidationResult, CredentialError>;
}

/// Account update (partial)
#[derive(Debug, Default)]
pub struct AccountUpdate {
    pub display_name: Option<String>,
    pub server_host: Option<String>,
    pub server_port: Option<u16>,
    pub status: Option<AccountStatus>,
}

/// Validation result for account
#[derive(Debug)]
pub struct ValidationResult {
    pub valid: bool,
    pub tls_version: Option<String>,
    pub certificate_valid: bool,
    pub certificate_expiry: Option<DateTime<Utc>>,
    pub errors: Vec<String>,
}
```

### 5.2 Tauri Commands

```rust
/// Tauri command: Store new SIP account
#[tauri::command]
pub async fn store_sip_account(
    state: State<'_, AppState>,
    display_name: String,
    server_host: String,
    server_port: u16,
    username: String,
    password: String,
) -> Result<String, String> {
    // Input validation
    validate_inputs(&display_name, &server_host, server_port, &username, &password)
        .map_err(|e| e.to_string())?;

    // Create account
    let account = SipAccount {
        id: Uuid::new_v4().to_string(),
        display_name,
        server_host,
        server_port,
        transport: SipTransport::Sips,
        status: AccountStatus::Active,
        created_at: Utc::now(),
        updated_at: Utc::now(),
    };

    // Create secure credentials
    let credentials = SipCredentials::new(username, password)
        .map_err(|e| e.to_string())?;

    // Store in service
    state.credential_service
        .store_credentials(account, credentials)
        .await
        .map_err(|e| e.to_string())
}

/// Tauri command: Get account list (no passwords)
#[tauri::command]
pub async fn list_sip_accounts(
    state: State<'_, AppState>,
) -> Result<Vec<SipAccount>, String> {
    state.credential_service
        .list_accounts()
        .await
        .map_err(|e| e.to_string())
}

/// Tauri command: Get credentials for registration
///
/// # Security Note
/// This command returns the password to frontend.
/// Frontend MUST:
/// - Use password immediately for SIP registration
/// - Never store in localStorage/sessionStorage
/// - Never log password value
/// - Clear from memory after use
#[tauri::command]
pub async fn get_sip_credentials(
    state: State<'_, AppState>,
    account_id: String,
) -> Result<SipCredentialsResponse, String> {
    let (account, credentials) = state.credential_service
        .get_credentials(&account_id)
        .await
        .map_err(|e| e.to_string())?;

    Ok(SipCredentialsResponse {
        account,
        username: credentials.username,
        password: credentials.password,
    })
    // credentials auto-zeroized on drop
}

/// Tauri command: Update account password
#[tauri::command]
pub async fn update_sip_password(
    state: State<'_, AppState>,
    account_id: String,
    new_password: String,
) -> Result<(), String> {
    // Get existing account for username
    let (account, _) = state.credential_service
        .get_credentials(&account_id)
        .await
        .map_err(|e| e.to_string())?;

    // Validate new password
    validate_password(&new_password).map_err(|e| e.to_string())?;

    // Create new credentials
    let credentials = SipCredentials::new(account.username, new_password)
        .map_err(|e| e.to_string())?;

    // Update in keychain
    state.credential_service
        .update_credentials(&account_id, credentials)
        .await
        .map_err(|e| e.to_string())
}

/// Tauri command: Delete account
#[tauri::command]
pub async fn delete_sip_account(
    state: State<'_, AppState>,
    account_id: String,
) -> Result<(), String> {
    state.credential_service
        .delete_account(&account_id)
        .await
        .map_err(|e| e.to_string())
}

/// Tauri command: Validate account connectivity
#[tauri::command]
pub async fn validate_sip_account(
    state: State<'_, AppState>,
    account_id: String,
) -> Result<ValidationResult, String> {
    state.credential_service
        .validate_account(&account_id)
        .await
        .map_err(|e| e.to_string())
}
```

### 5.3 Frontend API (TypeScript)

```typescript
// src/lib/api/credentials.ts

import { invoke } from '@tauri-apps/api/tauri';

export interface SipAccount {
  id: string;
  display_name: string;
  server_host: string;
  server_port: number;
  transport: 'SIPS';
  status: 'Active' | 'Disabled' | { ConfigError: string };
  created_at: string;
  updated_at: string;
}

export interface SipCredentialsResponse {
  account: SipAccount;
  username: string;
  password: string; // SECURITY: Never store, use immediately
}

export interface ValidationResult {
  valid: boolean;
  tls_version?: string;
  certificate_valid: boolean;
  certificate_expiry?: string;
  errors: string[];
}

/**
 * Store new SIP account credentials
 * @security Password stored in platform keychain, never in plaintext
 */
export async function storeSipAccount(
  displayName: string,
  serverHost: string,
  serverPort: number,
  username: string,
  password: string
): Promise<string> {
  return invoke<string>('store_sip_account', {
    displayName,
    serverHost,
    serverPort,
    username,
    password,
  });
}

/**
 * List all SIP accounts (metadata only, no passwords)
 */
export async function listSipAccounts(): Promise<SipAccount[]> {
  return invoke<SipAccount[]>('list_sip_accounts');
}

/**
 * Get credentials for SIP registration
 * @security Password returned for immediate use only. Never persist.
 */
export async function getSipCredentials(
  accountId: string
): Promise<SipCredentialsResponse> {
  return invoke<SipCredentialsResponse>('get_sip_credentials', { accountId });
}

/**
 * Update account password
 */
export async function updateSipPassword(
  accountId: string,
  newPassword: string
): Promise<void> {
  return invoke<void>('update_sip_password', { accountId, newPassword });
}

/**
 * Delete SIP account and credentials
 * @security Irreversible operation, prompts for confirmation
 */
export async function deleteSipAccount(accountId: string): Promise<void> {
  return invoke<void>('delete_sip_account', { accountId });
}

/**
 * Validate account can connect to SIP server
 */
export async function validateSipAccount(
  accountId: string
): Promise<ValidationResult> {
  return invoke<ValidationResult>('validate_sip_account', { accountId });
}
```

---

## 6. Edge Cases & Error Scenarios

### 6.1 Keychain Access Denied

**Scenario:** User denies OS keychain access permission

**Behavior:**
- Detect `KeychainError::PermissionDenied`
- Show user-friendly error: "RUSTALK needs keychain access to securely store SIP credentials. Please grant permission in System Preferences."
- Provide "Open System Preferences" button (macOS)
- Retry operation after permission granted

**Test:**
```rust
#[tokio::test]
async fn test_keychain_permission_denied() {
    let service = CredentialService::new_mock(MockKeyring::permission_denied());
    let result = service.store_credentials(account, credentials).await;

    assert!(matches!(result, Err(CredentialError::Keychain(
        KeychainError::PermissionDenied
    ))));
}
```

### 6.2 Keychain Unavailable

**Scenario:** Platform keychain service not running (rare)

**Behavior:**
- Detect `KeychainError::Unavailable`
- Log error with diagnostic info
- Show error: "System keychain unavailable. Try restarting RUSTALK."
- Attempt fallback to in-memory storage (session only, warn user)

**Test:**
```rust
#[tokio::test]
async fn test_keychain_unavailable_fallback() {
    let service = CredentialService::new_mock(MockKeyring::unavailable());
    let result = service.store_credentials(account, credentials).await;

    // Should fail gracefully, no panic
    assert!(result.is_err());
}
```

### 6.3 Duplicate Account

**Scenario:** User attempts to store account with existing ID

**Behavior:**
- Detect duplicate ID in database
- Return `CredentialError::AlreadyExists`
- Prompt user: "Account already exists. Update existing account?"
- Offer "Update" or "Create New" options

**Test:**
```rust
#[tokio::test]
async fn test_duplicate_account_id() {
    let service = setup_service().await;
    service.store_credentials(account.clone(), credentials.clone()).await.unwrap();

    let result = service.store_credentials(account, credentials).await;
    assert!(matches!(result, Err(CredentialError::AlreadyExists)));
}
```

### 6.4 Invalid Server Certificate

**Scenario:** SIP server presents invalid/expired certificate

**Behavior:**
- Detect during TLS handshake
- Return validation error with details
- Show error: "SIP server certificate invalid: {reason}. Contact your provider."
- Prevent connection (no insecure bypass)

**Test:**
```rust
#[tokio::test]
async fn test_invalid_certificate_rejected() {
    let service = setup_service().await;
    let account_id = service.store_credentials(account, credentials).await.unwrap();

    let validation = service.validate_account(&account_id).await.unwrap();
    assert!(!validation.valid);
    assert!(!validation.certificate_valid);
    assert!(validation.errors.contains(&"Certificate expired".to_string()));
}
```

### 6.5 Network Timeout During Validation

**Scenario:** Cannot connect to SIP server for validation

**Behavior:**
- Set timeout: 10 seconds
- Return partial validation result
- Show warning: "Cannot connect to SIP server. Check network connection."
- Allow saving account anyway (validate later)

**Test:**
```rust
#[tokio::test]
async fn test_validation_network_timeout() {
    let service = setup_service().await;
    let account_id = service.store_credentials(account, credentials).await.unwrap();

    let result = tokio::time::timeout(
        Duration::from_secs(10),
        service.validate_account(&account_id)
    ).await;

    assert!(result.is_ok()); // Should not hang forever
}
```

### 6.6 Password Too Long

**Scenario:** User enters password exceeding 256 characters

**Behavior:**
- Validate in `SipCredentials::new()`
- Return `ValidationError::PasswordTooLong`
- Show error: "Password too long (max 256 characters)."
- Truncate or reject input

**Test:**
```rust
#[test]
fn test_password_length_validation() {
    let long_password = "a".repeat(257);
    let result = SipCredentials::new("user".to_string(), long_password);

    assert!(matches!(result, Err(ValidationError::PasswordTooLong)));
}
```

### 6.7 SQL Injection Attempt

**Scenario:** User enters SQL injection in username field

**Behavior:**
- Use parameterized queries (never string concat)
- Sanitize all inputs with regex validation
- Reject suspicious characters
- Log security event

**Test:**
```rust
#[tokio::test]
async fn test_sql_injection_prevented() {
    let service = setup_service().await;
    let malicious_username = "admin'; DROP TABLE sip_accounts; --";

    let credentials = SipCredentials::new(malicious_username.to_string(), "pass".to_string());
    let result = service.store_credentials(account, credentials).await;

    assert!(matches!(result, Err(CredentialError::Validation(_))));

    // Verify table still exists
    let accounts = service.list_accounts().await.unwrap();
    assert!(accounts.is_empty()); // No data loss
}
```

### 6.8 Memory Dump Attack

**Scenario:** Attacker attempts memory dump to extract passwords

**Behavior:**
- Use `zeroize` crate for all password structs
- Zero-out on drop
- No password in panic messages
- No password in debug logs

**Test:**
```rust
#[test]
fn test_password_zeroized_on_drop() {
    let password = "secret123".to_string();
    let ptr = password.as_ptr();

    {
        let credentials = SipCredentials::new("user".to_string(), password).unwrap();
        // credentials dropped here
    }

    // Verify memory zeroed (unsafe, for testing only)
    unsafe {
        let slice = std::slice::from_raw_parts(ptr, 9);
        assert_eq!(slice, &[0u8; 9]); // All zeros
    }
}
```

---

## 7. Security Threat Model

### 7.1 Threat: Plaintext Credential Storage

**Attack Vector:** Attacker gains filesystem access, reads credentials

**Mitigation:**
- ✅ Store passwords only in platform keychain (encrypted by OS)
- ✅ Keychain requires OS authentication to access
- ✅ No credentials in SQLite database (metadata only)
- ✅ No credentials in log files

**Residual Risk:** Low (requires OS compromise)

### 7.2 Threat: Man-in-the-Middle (MITM) Attack

**Attack Vector:** Attacker intercepts SIP traffic, steals credentials

**Mitigation:**
- ✅ Enforce TLS/SIPS only (reject plaintext SIP)
- ✅ Use TLS 1.2+ with strong cipher suites
- ✅ Validate server certificates against system trust store
- ✅ Check certificate hostname matches server
- ✅ Reject expired certificates

**Residual Risk:** Low (requires certificate authority compromise)

### 7.3 Threat: Memory Dump/Core Dump

**Attack Vector:** Attacker triggers crash, extracts passwords from core dump

**Mitigation:**
- ✅ Use `zeroize` crate for password structs
- ✅ Zero-out credentials after use
- ✅ Never log passwords (even in debug mode)
- ✅ Prevent passwords in panic messages
- ✅ Disable core dumps in production builds

**Residual Risk:** Low (requires process compromise)

### 7.4 Threat: SQL Injection

**Attack Vector:** Attacker injects SQL via username/display name fields

**Mitigation:**
- ✅ Use parameterized queries (sqlx placeholders)
- ✅ Never concatenate user input into SQL
- ✅ Validate inputs with regex (whitelist approach)
- ✅ Limit string lengths
- ✅ Escape special characters

**Residual Risk:** Very Low (multiple layers of defense)

### 7.5 Threat: Keychain Malware

**Attack Vector:** Malware on system extracts keychain entries

**Mitigation:**
- ✅ Keychain entries require OS authentication
- ✅ Use application-specific service identifiers
- ✅ Recommend OS-level security (antivirus, firewall)
- ✅ Log keychain access attempts
- ⚠️ Limited: Cannot prevent privileged malware

**Residual Risk:** Medium (requires OS-level defense)

### 7.6 Threat: Shoulder Surfing

**Attack Vector:** Attacker observes user entering password

**Mitigation:**
- ✅ Use password input fields (hidden characters)
- ✅ Clear clipboard after paste (future: timeout)
- ⚠️ Limited: Cannot prevent physical observation

**Residual Risk:** Medium (user responsibility)

### 7.7 Threat: Replay Attack

**Attack Vector:** Attacker captures and replays SIP registration

**Mitigation:**
- ✅ SIP protocol includes nonce-based authentication (SIP digest)
- ✅ TLS prevents credential capture
- ✅ Server-side nonce validation
- ⚠️ Depends on SIP server implementation

**Residual Risk:** Low (protocol-level protection)

---

## 8. Platform-Specific Considerations

### 8.1 macOS

**Keychain Integration:**
- Use `Security.framework` via `keyring` crate
- Keychain items stored in login keychain
- Access requires user password or Touch ID
- Keychain automatically locked when system sleeps

**Code Signing:**
- Application must be signed for keychain access
- Use developer ID for production builds
- Entitlements: `keychain-access-groups`

**Permissions:**
- First keychain access prompts user approval
- User can deny access (handle gracefully)
- Revoked in System Preferences > Security > Privacy > Keychain

**Testing:**
```rust
#[cfg(target_os = "macos")]
#[tokio::test]
async fn test_macos_keychain_integration() {
    let service = CredentialService::new().await.unwrap();
    let account_id = service.store_credentials(account, credentials).await.unwrap();

    // Verify stored in macOS keychain
    let (_, retrieved) = service.get_credentials(&account_id).await.unwrap();
    assert_eq!(retrieved.username, "testuser");
}
```

### 8.2 Windows

**Credential Manager Integration:**
- Use Windows Credential Manager API via `keyring` crate
- Credentials stored per-user
- Access requires Windows user authentication
- Encrypted using DPAPI (Data Protection API)

**UAC Considerations:**
- No UAC prompt for credential access
- User-level credentials (not system-level)

**Permissions:**
- No explicit permission prompt
- Credentials accessible by user's applications
- Protected by Windows user account

**Testing:**
```rust
#[cfg(target_os = "windows")]
#[tokio::test]
async fn test_windows_credential_manager_integration() {
    let service = CredentialService::new().await.unwrap();
    let account_id = service.store_credentials(account, credentials).await.unwrap();

    // Verify stored in Windows Credential Manager
    let (_, retrieved) = service.get_credentials(&account_id).await.unwrap();
    assert_eq!(retrieved.username, "testuser");
}
```

### 8.3 Linux (Future)

**Secret Service Integration:**
- Use freedesktop.org Secret Service API
- Backends: GNOME Keyring, KDE Wallet
- Requires D-Bus session bus
- May not be available in all environments

**Fallback:**
- Encrypted file storage (libsodium)
- Warn user about reduced security

---

## 9. Test Scenarios

### 9.1 Unit Tests

**Rust Backend:**
```rust
// tests/credential_service_test.rs

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_store_and_retrieve_credentials() {
        // Arrange
        let service = CredentialService::new_test().await.unwrap();
        let account = create_test_account();
        let credentials = create_test_credentials();

        // Act
        let account_id = service.store_credentials(account.clone(), credentials.clone())
            .await.unwrap();
        let (retrieved_account, retrieved_creds) = service.get_credentials(&account_id)
            .await.unwrap();

        // Assert
        assert_eq!(retrieved_account.id, account.id);
        assert_eq!(retrieved_creds.username, credentials.username);
        assert_eq!(retrieved_creds.password, credentials.password);
    }

    #[tokio::test]
    async fn test_update_password() {
        let service = setup_service().await;
        let account_id = service.store_credentials(account, credentials).await.unwrap();

        let new_password = "newpassword123";
        let new_creds = SipCredentials::new("user".to_string(), new_password.to_string()).unwrap();

        service.update_credentials(&account_id, new_creds).await.unwrap();

        let (_, retrieved) = service.get_credentials(&account_id).await.unwrap();
        assert_eq!(retrieved.password, new_password);
    }

    #[tokio::test]
    async fn test_delete_account_removes_keychain_entry() {
        let service = setup_service().await;
        let account_id = service.store_credentials(account, credentials).await.unwrap();

        service.delete_account(&account_id).await.unwrap();

        let result = service.get_credentials(&account_id).await;
        assert!(matches!(result, Err(CredentialError::NotFound)));
    }

    #[tokio::test]
    async fn test_list_accounts_excludes_passwords() {
        let service = setup_service().await;
        service.store_credentials(account1, credentials1).await.unwrap();
        service.store_credentials(account2, credentials2).await.unwrap();

        let accounts = service.list_accounts().await.unwrap();

        assert_eq!(accounts.len(), 2);
        // Verify no password field exists in SipAccount struct
    }

    #[test]
    fn test_credentials_zeroized_on_drop() {
        let password = "secret123".to_string();
        let credentials = SipCredentials::new("user".to_string(), password).unwrap();

        drop(credentials);

        // Password should be zeroed in memory
        // (verified via valgrind/asan in CI)
    }

    #[test]
    fn test_invalid_port_rejected() {
        let result = validate_server_port(0);
        assert!(matches!(result, Err(ValidationError::InvalidPort)));

        let result = validate_server_port(65536);
        assert!(matches!(result, Err(ValidationError::InvalidPort)));
    }

    #[test]
    fn test_control_characters_rejected() {
        let malicious_host = "server.com\n\r\0";
        let result = validate_server_host(malicious_host);
        assert!(matches!(result, Err(ValidationError::InvalidCharacters)));
    }

    #[tokio::test]
    async fn test_tls_enforcement() {
        let service = setup_service().await;
        let mut account = create_test_account();
        account.server_port = 5060; // Non-TLS port

        let result = service.validate_account(&account.id).await.unwrap();

        assert!(!result.valid);
        assert!(result.errors.contains(&"Non-TLS port detected".to_string()));
    }

    #[tokio::test]
    async fn test_certificate_expiry_detected() {
        let service = setup_service_with_mock_tls().await;
        let account_id = service.store_credentials(account, credentials).await.unwrap();

        // Mock: server certificate expires in 1 day
        let validation = service.validate_account(&account_id).await.unwrap();

        assert!(validation.certificate_valid);
        assert!(validation.certificate_expiry.unwrap() < Utc::now() + Duration::days(7));
    }
}
```

### 9.2 Integration Tests

**Tauri Command Tests:**
```rust
// tests/tauri_commands_test.rs

#[cfg(test)]
mod integration_tests {
    use super::*;

    #[tokio::test]
    async fn test_store_account_command() {
        let app = create_test_app().await;

        let result = store_sip_account(
            app.state(),
            "Work Phone".to_string(),
            "sip.example.com".to_string(),
            5061,
            "user@example.com".to_string(),
            "password123".to_string(),
        ).await;

        assert!(result.is_ok());
        let account_id = result.unwrap();
        assert!(!account_id.is_empty());
    }

    #[tokio::test]
    async fn test_list_accounts_command() {
        let app = create_test_app().await;
        store_test_accounts(&app, 3).await;

        let result = list_sip_accounts(app.state()).await;

        assert!(result.is_ok());
        assert_eq!(result.unwrap().len(), 3);
    }

    #[tokio::test]
    async fn test_get_credentials_command_requires_auth() {
        let app = create_test_app().await;
        let account_id = store_test_account(&app).await;

        // This will prompt OS keychain authentication
        let result = get_sip_credentials(app.state(), account_id).await;

        assert!(result.is_ok());
        let response = result.unwrap();
        assert_eq!(response.username, "testuser");
        assert!(!response.password.is_empty());
    }

    #[tokio::test]
    async fn test_delete_account_command() {
        let app = create_test_app().await;
        let account_id = store_test_account(&app).await;

        let result = delete_sip_account(app.state(), account_id.clone()).await;
        assert!(result.is_ok());

        // Verify deleted
        let get_result = get_sip_credentials(app.state(), account_id).await;
        assert!(get_result.is_err());
    }
}
```

### 9.3 End-to-End Tests

**Playwright E2E:**
```typescript
// tests/e2e/credentials.spec.ts

import { test, expect } from '@playwright/test';

test.describe('SIP Credential Management', () => {
  test('should store new account', async ({ page }) => {
    await page.goto('/settings/accounts');
    await page.click('button:has-text("Add Account")');

    await page.fill('input[name="displayName"]', 'Work Phone');
    await page.fill('input[name="serverHost"]', 'sip.example.com');
    await page.fill('input[name="serverPort"]', '5061');
    await page.fill('input[name="username"]', 'user@example.com');
    await page.fill('input[name="password"]', 'password123');

    await page.click('button:has-text("Save")');

    // Should show success message
    await expect(page.locator('.success-message')).toContainText('Account saved');

    // Should appear in account list
    await expect(page.locator('.account-item')).toContainText('Work Phone');
  });

  test('should validate certificate on save', async ({ page }) => {
    await page.goto('/settings/accounts');
    await page.click('button:has-text("Add Account")');

    // Fill form with invalid server
    await page.fill('input[name="serverHost"]', 'invalid.example.com');
    await page.fill('input[name="serverPort"]', '5061');
    await page.fill('input[name="username"]', 'user@example.com');
    await page.fill('input[name="password"]', 'password123');

    await page.click('button:has-text("Save")');

    // Should show certificate error
    await expect(page.locator('.error-message')).toContainText('Certificate invalid');
  });

  test('should prompt OS keychain on credential access', async ({ page }) => {
    // Pre-create account
    const accountId = await createTestAccount();

    await page.goto(`/calls?account=${accountId}`);

    // On macOS, this triggers keychain prompt
    // Test should wait for manual approval or mock approval

    await expect(page.locator('.status')).toContainText('Connected');
  });

  test('should delete account with confirmation', async ({ page }) => {
    const accountId = await createTestAccount();

    await page.goto('/settings/accounts');
    await page.click(`[data-account-id="${accountId}"] button:has-text("Delete")`);

    // Should show confirmation dialog
    await expect(page.locator('.confirm-dialog')).toBeVisible();
    await page.click('.confirm-dialog button:has-text("Delete")');

    // Should remove from list
    await expect(page.locator(`[data-account-id="${accountId}"]`)).not.toBeVisible();
  });

  test('should handle keychain permission denied', async ({ page }) => {
    // Mock keychain denial
    await mockKeychainDenied();

    await page.goto('/settings/accounts');
    await page.click('button:has-text("Add Account")');
    // Fill and save account...

    // Should show permission error
    await expect(page.locator('.error-message')).toContainText('keychain access');
    await expect(page.locator('button:has-text("Open System Preferences")')).toBeVisible();
  });
});
```

### 9.4 Security Tests

**Automated Security Scanning:**
```bash
# Cargo audit (dependency vulnerabilities)
cargo audit

# Clippy (code quality + security lints)
cargo clippy -- -D warnings

# Memory safety checks (valgrind/asan)
RUSTFLAGS="-Z sanitizer=address" cargo test

# SQL injection tests
cargo test test_sql_injection

# Memory leak tests
cargo test --features leak-detector
```

---

## 10. Implementation Checklist

### Phase 1: Foundation (Week 1)
- [ ] Setup `keyring` crate integration
- [ ] Create `SipAccount` and `SipCredentials` structs
- [ ] Implement `CredentialService` skeleton
- [ ] Setup SQLite database schema
- [ ] Write unit tests for data models

### Phase 2: Storage Operations (Week 1-2)
- [ ] Implement `store_credentials()`
- [ ] Implement `get_credentials()`
- [ ] Implement `update_credentials()`
- [ ] Implement `delete_account()`
- [ ] Implement `list_accounts()`
- [ ] Add input validation functions
- [ ] Write unit tests (85%+ coverage)

### Phase 3: Security Features (Week 2)
- [ ] Integrate `zeroize` for password structs
- [ ] Implement TLS/SIPS enforcement
- [ ] Add certificate validation logic
- [ ] Setup audit logging
- [ ] Write security tests

### Phase 4: Tauri Integration (Week 2-3)
- [ ] Create Tauri commands for all operations
- [ ] Add error handling and conversion
- [ ] Write integration tests
- [ ] Test on macOS and Windows

### Phase 5: Frontend API (Week 3)
- [ ] Create TypeScript API wrapper
- [ ] Implement frontend error handling
- [ ] Add loading states
- [ ] Write E2E tests

### Phase 6: Testing & Hardening (Week 3-4)
- [ ] Run security audit (cargo audit)
- [ ] Memory leak testing
- [ ] Platform-specific testing (macOS + Windows)
- [ ] Performance benchmarks
- [ ] Manual security review

### Phase 7: Documentation (Week 4)
- [ ] API documentation (rustdoc)
- [ ] Security best practices guide
- [ ] User guide for credential management
- [ ] Troubleshooting guide

---

## 11. Success Criteria

### 11.1 Functional
- ✅ Store, retrieve, update, delete credentials work on macOS and Windows
- ✅ Multiple accounts supported (10+ tested)
- ✅ TLS/SIPS enforcement prevents insecure connections
- ✅ Certificate validation detects invalid certificates
- ✅ Input validation rejects malicious inputs

### 11.2 Security
- ✅ 0 credentials stored in plaintext
- ✅ Pass `cargo audit` with 0 vulnerabilities
- ✅ Pass memory safety tests (asan, valgrind)
- ✅ Credentials zeroized in memory after use
- ✅ No passwords in logs or panic messages

### 11.3 Testing
- ✅ 85%+ test coverage for Rust backend
- ✅ 80%+ test coverage for TypeScript frontend
- ✅ All E2E scenarios pass on macOS and Windows
- ✅ Security tests detect common vulnerabilities

### 11.4 Performance
- ✅ <50ms credential retrieval (p95)
- ✅ <100ms credential storage (p95)
- ✅ <10s account validation (p95)

### 11.5 Usability
- ✅ Clear error messages for all failure scenarios
- ✅ Native OS dialogs for keychain access
- ✅ Account setup completes in <60 seconds

---

## 12. Dependencies

### 12.1 Rust Crates

```toml
[dependencies]
# Keychain access
keyring = "2.0"

# TLS and certificate validation
rustls = "0.21"
webpki-roots = "0.25"
rustls-native-certs = "0.6"

# Secure memory
zeroize = { version = "1.6", features = ["derive"] }

# Database
sqlx = { version = "0.7", features = ["sqlite", "runtime-tokio-rustls"] }

# Async runtime
tokio = { version = "1.35", features = ["full"] }

# Serialization
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"

# UUID generation
uuid = { version = "1.6", features = ["v4", "serde"] }

# Date/time
chrono = { version = "0.4", features = ["serde"] }

# Error handling
thiserror = "1.0"
anyhow = "1.0"

# Validation
regex = "1.10"
validator = { version = "0.16", features = ["derive"] }

# Logging
tracing = "0.1"
tracing-subscriber = "0.3"

[dev-dependencies]
# Testing
tokio-test = "0.4"
mockall = "0.12"
proptest = "1.4"

# Benchmarking
criterion = "0.5"
```

### 12.2 Platform Dependencies

**macOS:**
- Xcode Command Line Tools (for Security.framework)
- macOS 11+ SDK

**Windows:**
- Windows 10 SDK
- Visual Studio Build Tools 2019+

---

## 13. Migration & Rollout

### 13.1 Initial Release (v1.0.0)
- Fresh installs only
- No migration needed
- Feature flag: `security_storage` (enabled by default)

### 13.2 Future Updates

**v1.1.0: Biometric Authentication**
- Migrate existing accounts (no action required)
- Add Touch ID/Windows Hello support
- Backward compatible

**v1.2.0: Cloud Sync (Optional)**
- Opt-in feature
- End-to-end encrypted sync
- Requires user consent

---

## 14. Open Questions

1. **Self-Signed Certificates:**
   - Should we support self-signed certificates for development?
   - Proposal: Add "Trust Certificate" button with strong warning

2. **Credential Import:**
   - Should MVP support importing credentials from file?
   - Proposal: Post-MVP, requires secure format definition

3. **Master Password:**
   - Should we add optional master password layer?
   - Proposal: Post-MVP, survey users first

4. **Cloud Backup:**
   - Should credentials be backed up to cloud (iCloud/OneDrive)?
   - Proposal: Post-MVP, requires end-to-end encryption

---

## 15. References

- [OWASP Secure Coding Practices](https://owasp.org/www-project-secure-coding-practices-quick-reference-guide/)
- [Tauri Security Best Practices](https://tauri.app/v1/references/architecture/security)
- [macOS Keychain Services](https://developer.apple.com/documentation/security/keychain_services)
- [Windows Credential Manager](https://docs.microsoft.com/en-us/windows/win32/secauthn/credential-manager)
- [RFC 3261 - SIP Protocol](https://datatracker.ietf.org/doc/html/rfc3261)
- [RFC 5246 - TLS 1.2](https://datatracker.ietf.org/doc/html/rfc5246)

---

## 16. Approval

**Specification Review:**
- [ ] Security Team Review
- [ ] Architecture Team Review
- [ ] Product Team Review

**Approval Date:** _Pending_
**Approved By:** _Pending_

---

**Document Version History:**

| Version | Date       | Author | Changes                |
|---------|------------|--------|------------------------|
| 1.0.0   | 2025-10-11 | SPARC  | Initial specification |

---

**End of Specification**
