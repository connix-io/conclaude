/// Database connection management
pub mod connection;

/// Database entities (models)
pub mod entities;

/// Database migrations
pub mod migrations;

// Re-export commonly used types and functions for cleaner API
pub use connection::{get_connection, get_database_path};
#[allow(unused_imports)]
pub use entities::{HookExecution, HookExecutionActiveModel};
