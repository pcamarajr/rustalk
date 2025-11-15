// Domain traits - Dependency inversion interfaces
// Infrastructure layer implements these traits

pub mod audio_engine;
pub mod credential_store;

pub use audio_engine::AudioEngine;
pub use credential_store::CredentialStore;
