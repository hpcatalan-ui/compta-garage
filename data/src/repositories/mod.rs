//! Repositories module

pub mod client_repository;
pub mod vehicle_repository;
pub mod supplier_repository;
pub mod part_repository;
pub mod invoice_repository;
pub mod payment_repository;
pub mod user_repository;

// Re-export all repositories
pub use client_repository::*;
pub use vehicle_repository::*;
pub use supplier_repository::*;
pub use part_repository::*;
pub use invoice_repository::*;
pub use payment_repository::*;
pub use user_repository::*;
