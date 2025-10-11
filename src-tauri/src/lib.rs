// Library entry point for testing
// This allows us to write tests that depend on internal modules

pub mod domain;
pub mod services;
pub mod commands;
pub mod infrastructure;

// Re-export common types for testing
pub use domain::*;
