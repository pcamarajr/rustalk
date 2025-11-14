// Domain layer - Core business entities and traits
// This layer has zero external dependencies (except async-trait and thiserror for errors)

pub mod entities;
pub mod errors;
pub mod traits;

pub use entities::Credentials;
pub use errors::CredentialStoreError;
pub use traits::CredentialStore;

