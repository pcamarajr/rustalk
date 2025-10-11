// Domain Layer - Core business logic (framework-agnostic)
//
// This layer contains:
// - Pure business entities (no external dependencies)
// - Domain events
// - Trait definitions for infrastructure
//
// Dependencies: NONE (pure Rust only)

pub mod entities;
pub mod events;
pub mod traits;

// Re-export commonly used types
pub use entities::*;
pub use events::*;
pub use traits::*;
