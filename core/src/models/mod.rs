//! Domain models for Garage Compta

pub mod address;
pub mod client;
pub mod vehicle;
pub mod supplier;
pub mod part;
pub mod invoice;
pub mod payment;
pub mod user;

// Re-export all types
pub use address::*;
pub use client::*;
pub use vehicle::*;
pub use supplier::*;
pub use part::*;
pub use invoice::*;
pub use payment::*;
pub use user::*;
