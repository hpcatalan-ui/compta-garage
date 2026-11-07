//! API routes

pub mod clients;
pub mod vehicles;
pub mod invoices;
pub mod parts;
pub mod suppliers;
pub mod auth;
pub mod health;

use axum::Router;

use crate::state::AppState;

/// Creates the main router
pub fn create_router(state: AppState) -> Router<AppState> {
    let mut router = Router::new();
    
    // Add routes - each router needs the state
    router = router.merge(health::router(state.clone()));
    router = router.merge(auth::router(state.clone()));
    router = router.merge(clients::router(state.clone()));
    router = router.merge(vehicles::router(state.clone()));
    router = router.merge(invoices::router(state.clone()));
    router = router.merge(parts::router(state.clone()));
    router = router.merge(suppliers::router(state.clone()));
    
    // Apply global middleware
    router = router.layer(axum::middleware::from_fn(crate::middleware::logging::logging_middleware));
    
    router
}

/// Creates the Axum application
pub async fn create_app(state: AppState) -> Router<AppState> {
    create_router(state)
}
