//! Part routes

use axum::{Router, routing::{get, post, put, delete}};
use axum::extract::{State, Path, Query};
use axum::Json;
use uuid::Uuid;
use garage_compta_core::models::{Part, PartCategory};
use garage_compta_data::repositories::PartRepository;
use crate::state::AppState;
use crate::middleware::ApiError;

/// Creates the part router
pub fn router(state: AppState) -> Router<AppState> {
    Router::new().with_state(state)
        .route("/", get(get_parts))
        .route("/", post(create_part))
        .route("/:id", get(get_part))
        .route("/:id", put(update_part))
        .route("/:id", delete(delete_part))
        .route("/category/:category", get(get_parts_by_category))
        .route("/reference/:reference", get(get_part_by_reference))
        .route("/below-minimum", get(get_parts_below_minimum))
        .route("/search", get(search_parts))
}

/// Gets all parts
async fn get_parts(State(state): State<AppState>) -> Result<Json<Vec<Part>>, ApiError> {
    let repo = PartRepository::new(state.db.pool.clone());
    let parts = repo.get_all().await?;
    Ok(Json(parts))
}

/// Gets a part by ID
async fn get_part(Path(id): Path<Uuid>, State(state): State<AppState>) -> Result<Json<Part>, ApiError> {
    let repo = PartRepository::new(state.db.pool.clone());
    let part = repo.get(id).await?.ok_or_else(|| {
        ApiError::new(axum::http::StatusCode::NOT_FOUND, "Part not found")
    })?;
    Ok(Json(part))
}

/// Creates a new part
async fn create_part(State(state): State<AppState>, Json(part): Json<Part>) -> Result<Json<Part>, ApiError> {
    let repo = PartRepository::new(state.db.pool.clone());
    let part = repo.create(&part).await?;
    Ok(Json(part))
}

/// Updates a part
async fn update_part(Path(id): Path<Uuid>, State(state): State<AppState>, Json(part): Json<Part>) -> Result<Json<Part>, ApiError> {
    let repo = PartRepository::new(state.db.pool.clone());
    let mut part = part;
    part.id = id;
    part.updated_at = chrono::Utc::now();
    let part = repo.update(&part).await?;
    Ok(Json(part))
}

/// Deletes a part
async fn delete_part(Path(id): Path<Uuid>, State(state): State<AppState>) -> Result<Json<bool>, ApiError> {
    let repo = PartRepository::new(state.db.pool.clone());
    let deleted = repo.delete(id).await?;
    Ok(Json(deleted))
}

/// Gets parts by category
async fn get_parts_by_category(Path(category): Path<String>, State(state): State<AppState>) -> Result<Json<Vec<Part>>, ApiError> {
    let repo = PartRepository::new(state.db.pool.clone());
    let category_enum: PartCategory = category.parse().map_err(|_| {
        ApiError::new(axum::http::StatusCode::BAD_REQUEST, "Invalid category")
    })?;
    let parts = repo.get_by_category(category_enum).await?;
    Ok(Json(parts))
}

/// Gets a part by reference
async fn get_part_by_reference(Path(reference): Path<String>, State(state): State<AppState>) -> Result<Json<Part>, ApiError> {
    let repo = PartRepository::new(state.db.pool.clone());
    let part = repo.get_by_reference(&reference).await?.ok_or_else(|| {
        ApiError::new(axum::http::StatusCode::NOT_FOUND, "Part not found")
    })?;
    Ok(Json(part))
}

/// Gets parts below minimum stock
async fn get_parts_below_minimum(State(state): State<AppState>) -> Result<Json<Vec<Part>>, ApiError> {
    let repo = PartRepository::new(state.db.pool.clone());
    let parts = repo.get_below_minimum().await?;
    Ok(Json(parts))
}

/// Searches parts
async fn search_parts(Query(query): Query<String>, State(state): State<AppState>) -> Result<Json<Vec<Part>>, ApiError> {
    let repo = PartRepository::new(state.db.pool.clone());
    let parts = repo.search(&query).await?;
    Ok(Json(parts))
}
