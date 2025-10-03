// Library entry point for testing
// This allows us to write tests that depend on internal modules

pub mod domain;

// Re-export common types for testing
pub use domain::*;
