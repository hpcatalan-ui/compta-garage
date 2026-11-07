//! Garage Compta - Data Access Layer
//!
//! This module handles database access:
//! - Connection management
//! - Repositories for each model
//! - Database migrations

pub mod connection;
pub mod migrations;
pub mod repositories;

// Re-export main types
pub use connection::*;
pub use repositories::*;
