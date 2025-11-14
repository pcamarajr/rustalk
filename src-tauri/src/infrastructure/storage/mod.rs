// Storage implementations - Platform-specific credential storage

#[cfg(target_os = "macos")]
pub mod keychain;

#[cfg(target_os = "macos")]
pub use keychain::KeychainCredentialStore;
