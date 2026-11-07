//! Garage Compta - API REST
//!
//! This module exposes a REST API for the Garage Compta application.
//! It uses Axum as the web framework.

pub mod middleware;
pub mod routes;
pub mod state;

// Re-export main types
pub use state::*;
pub use routes::*;
