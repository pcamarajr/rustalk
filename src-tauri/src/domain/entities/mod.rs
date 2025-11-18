// Domain entities - Core business value objects and entities

pub mod credentials;
pub mod registration;

pub use credentials::{Credentials, TransportProtocol};
pub use registration::{Registration, RegistrationState};
