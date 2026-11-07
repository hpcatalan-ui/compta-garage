//! Vehicle routes

use axum::{Router, routing::{get, post, put, delete}};
use axum::extract::{State, Path};
use axum::Json;
use uuid::Uuid;
use garage_compta_core::models::Vehicle;
use garage_compta_data::repositories::VehicleRepository;
use crate::state::AppState;
use crate::middleware::ApiError;

/// Creates the vehicle router
pub fn router(state: AppState) -> Router<AppState> {
    Router::new().with_state(state)
        .route("/", get(get_vehicles))
        .route("/", post(create_vehicle))
        .route("/:id", get(get_vehicle))
        .route("/:id", put(update_vehicle))
        .route("/:id", delete(delete_vehicle))
        .route("/owner/:owner_id", get(get_vehicles_by_owner))
        .route("/registration/:registration", get(get_vehicle_by_registration))
}

/// Gets all vehicles
async fn get_vehicles(State(state): State<AppState>) -> Result<Json<Vec<Vehicle>>, ApiError> {
    let repo = VehicleRepository::new(state.db.pool.clone());
    let vehicles = repo.get_all().await?;
    Ok(Json(vehicles))
}

/// Gets a vehicle by ID
async fn get_vehicle(Path(id): Path<Uuid>, State(state): State<AppState>) -> Result<Json<Vehicle>, ApiError> {
    let repo = VehicleRepository::new(state.db.pool.clone());
    let vehicle = repo.get(id).await?.ok_or_else(|| {
        ApiError::new(axum::http::StatusCode::NOT_FOUND, "Vehicle not found")
    })?;
    Ok(Json(vehicle))
}

/// Creates a new vehicle
async fn create_vehicle(State(state): State<AppState>, Json(vehicle): Json<Vehicle>) -> Result<Json<Vehicle>, ApiError> {
    let repo = VehicleRepository::new(state.db.pool.clone());
    let vehicle = repo.create(&vehicle).await?;
    Ok(Json(vehicle))
}

/// Updates a vehicle
async fn update_vehicle(Path(id): Path<Uuid>, State(state): State<AppState>, Json(vehicle): Json<Vehicle>) -> Result<Json<Vehicle>, ApiError> {
    let repo = VehicleRepository::new(state.db.pool.clone());
    let mut vehicle = vehicle;
    vehicle.id = id;
    vehicle.updated_at = chrono::Utc::now();
    let vehicle = repo.update(&vehicle).await?;
    Ok(Json(vehicle))
}

/// Deletes a vehicle
async fn delete_vehicle(Path(id): Path<Uuid>, State(state): State<AppState>) -> Result<Json<bool>, ApiError> {
    let repo = VehicleRepository::new(state.db.pool.clone());
    let deleted = repo.delete(id).await?;
    Ok(Json(deleted))
}

/// Gets vehicles by owner
async fn get_vehicles_by_owner(Path(owner_id): Path<Uuid>, State(state): State<AppState>) -> Result<Json<Vec<Vehicle>>, ApiError> {
    let repo = VehicleRepository::new(state.db.pool.clone());
    let vehicles = repo.get_by_owner(owner_id).await?;
    Ok(Json(vehicles))
}

/// Gets a vehicle by registration
async fn get_vehicle_by_registration(Path(registration): Path<String>, State(state): State<AppState>) -> Result<Json<Vehicle>, ApiError> {
    let repo = VehicleRepository::new(state.db.pool.clone());
    let vehicle = repo.get_by_registration(&registration).await?.ok_or_else(|| {
        ApiError::new(axum::http::StatusCode::NOT_FOUND, "Vehicle not found")
    })?;
    Ok(Json(vehicle))
}
