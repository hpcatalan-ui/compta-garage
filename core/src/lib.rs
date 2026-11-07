//! Garage Compta - Core Module
//!
//! This module contains the domain models, business logic, and core functionality
//! for the Garage Compta application.

pub mod error;
pub mod models;
pub mod sqlx_impl;

// Re-export main types
pub use error::*;
pub use models::*;
