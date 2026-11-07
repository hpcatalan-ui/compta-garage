//! Supplier routes

use axum::{Router, routing::{get, post, put, delete}};
use axum::extract::{State, Path, Query};
use axum::Json;
use uuid::Uuid;
use garage_compta_core::models::Supplier;
use garage_compta_data::repositories::SupplierRepository;
use crate::state::AppState;
use crate::middleware::ApiError;

/// Creates the supplier router
pub fn router(state: AppState) -> Router<AppState> {
    Router::new().with_state(state)
        .route("/", get(get_suppliers))
        .route("/", post(create_supplier))
        .route("/:id", get(get_supplier))
        .route("/:id", put(update_supplier))
        .route("/:id", delete(delete_supplier))
        .route("/search", get(search_suppliers))
        .route("/active", get(get_active_suppliers))
}

/// Gets all suppliers
async fn get_suppliers(State(state): State<AppState>) -> Result<Json<Vec<Supplier>>, ApiError> {
    let repo = SupplierRepository::new(state.db.pool.clone());
    let suppliers = repo.get_all().await?;
    Ok(Json(suppliers))
}

/// Gets a supplier by ID
async fn get_supplier(Path(id): Path<Uuid>, State(state): State<AppState>) -> Result<Json<Supplier>, ApiError> {
    let repo = SupplierRepository::new(state.db.pool.clone());
    let supplier = repo.get(id).await?.ok_or_else(|| {
        ApiError::new(axum::http::StatusCode::NOT_FOUND, "Supplier not found")
    })?;
    Ok(Json(supplier))
}

/// Creates a new supplier
async fn create_supplier(State(state): State<AppState>, Json(supplier): Json<Supplier>) -> Result<Json<Supplier>, ApiError> {
    let repo = SupplierRepository::new(state.db.pool.clone());
    let supplier = repo.create(&supplier).await?;
    Ok(Json(supplier))
}

/// Updates a supplier
async fn update_supplier(Path(id): Path<Uuid>, State(state): State<AppState>, Json(supplier): Json<Supplier>) -> Result<Json<Supplier>, ApiError> {
    let repo = SupplierRepository::new(state.db.pool.clone());
    let mut supplier = supplier;
    supplier.id = id;
    supplier.updated_at = chrono::Utc::now();
    let supplier = repo.update(&supplier).await?;
    Ok(Json(supplier))
}

/// Deletes a supplier
async fn delete_supplier(Path(id): Path<Uuid>, State(state): State<AppState>) -> Result<Json<bool>, ApiError> {
    let repo = SupplierRepository::new(state.db.pool.clone());
    let deleted = repo.delete(id).await?;
    Ok(Json(deleted))
}

/// Searches suppliers
async fn search_suppliers(Query(query): Query<String>, State(state): State<AppState>) -> Result<Json<Vec<Supplier>>, ApiError> {
    let repo = SupplierRepository::new(state.db.pool.clone());
    let suppliers = repo.search(&query).await?;
    Ok(Json(suppliers))
}

/// Gets active suppliers
async fn get_active_suppliers(State(state): State<AppState>) -> Result<Json<Vec<Supplier>>, ApiError> {
    let repo = SupplierRepository::new(state.db.pool.clone());
    let suppliers = repo.get_active().await?;
    Ok(Json(suppliers))
}
