// Domain layer - Core business entities and traits
// This layer has minimal external dependencies (async-trait, thiserror, serde)

pub mod entities;
pub mod errors;
pub mod traits;

pub use entities::Credentials;
pub use errors::CredentialStoreError;
pub use traits::CredentialStore;
