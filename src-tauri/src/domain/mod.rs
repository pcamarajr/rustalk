// Domain layer - Core business entities and traits
// This layer has minimal external dependencies (async-trait, thiserror, serde)

pub mod entities;
pub mod errors;
pub mod traits;

pub use entities::{Credentials, Registration, RegistrationState};
pub use errors::{AudioEngineError, CommandError, CredentialStoreError, SipError};
pub use traits::{AudioEngine, CredentialStore};
