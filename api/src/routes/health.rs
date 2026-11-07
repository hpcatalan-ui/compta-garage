//! Health check routes

use axum::{Router, routing::get, Json, http::StatusCode};
use serde_json::json;
use crate::state::AppState;

/// Creates the health router
pub fn router(state: AppState) -> Router<AppState> {
    Router::new().with_state(state)
        .route("/health", get(health_check))
        .route("/ready", get(ready_check))
}

/// Health check endpoint
async fn health_check() -> (StatusCode, Json<serde_json::Value>) {
    (
        StatusCode::OK,
        Json(json!({
            "status": "healthy",
            "timestamp": chrono::Utc::now().to_rfc3339()
        })),
    )
}

/// Readiness check endpoint
async fn ready_check() -> (StatusCode, Json<serde_json::Value>) {
    (
        StatusCode::OK,
        Json(json!({
            "status": "ready",
            "timestamp": chrono::Utc::now().to_rfc3339()
        })),
    )
}
