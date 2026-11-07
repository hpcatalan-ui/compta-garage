//! Client routes

use axum::{Router, routing::{get, post, put, delete}};
use axum::extract::{State, Path, Query};
use axum::Json;
use uuid::Uuid;
use garage_compta_core::models::Client;
use garage_compta_data::repositories::ClientRepository;
use crate::state::AppState;
use crate::middleware::ApiError;

/// Creates the client router
pub fn router(state: AppState) -> Router<AppState> {
    Router::new().with_state(state)
        .route("/", get(get_clients))
        .route("/", post(create_client))
        .route("/:id", get(get_client))
        .route("/:id", put(update_client))
        .route("/:id", delete(delete_client))
        .route("/search", get(search_clients))
}

/// Gets all clients
async fn get_clients(State(state): State<AppState>) -> Result<Json<Vec<Client>>, ApiError> {
    let repo = ClientRepository::new(state.db.pool.clone());
    let clients = repo.get_all().await?;
    Ok(Json(clients))
}

/// Gets a client by ID
async fn get_client(Path(id): Path<Uuid>, State(state): State<AppState>) -> Result<Json<Client>, ApiError> {
    let repo = ClientRepository::new(state.db.pool.clone());
    let client = repo.get(id).await?.ok_or_else(|| {
        ApiError::new(axum::http::StatusCode::NOT_FOUND, "Client not found")
    })?;
    Ok(Json(client))
}

/// Creates a new client
async fn create_client(State(state): State<AppState>, Json(client): Json<Client>) -> Result<Json<Client>, ApiError> {
    let repo = ClientRepository::new(state.db.pool.clone());
    let client = repo.create(&client).await?;
    Ok(Json(client))
}

/// Updates a client
async fn update_client(Path(id): Path<Uuid>, State(state): State<AppState>, Json(client): Json<Client>) -> Result<Json<Client>, ApiError> {
    let repo = ClientRepository::new(state.db.pool.clone());
    let _existing = repo.get(id).await?.ok_or_else(|| {
        ApiError::new(axum::http::StatusCode::NOT_FOUND, "Client not found")
    })?;
    
    let mut client = client;
    client.id = id;
    client.updated_at = chrono::Utc::now();
    
    let client = repo.update(&client).await?;
    Ok(Json(client))
}

/// Deletes a client
async fn delete_client(Path(id): Path<Uuid>, State(state): State<AppState>) -> Result<Json<bool>, ApiError> {
    let repo = ClientRepository::new(state.db.pool.clone());
    let deleted = repo.delete(id).await?;
    Ok(Json(deleted))
}

/// Searches clients
async fn search_clients(Query(query): Query<String>, State(state): State<AppState>) -> Result<Json<Vec<Client>>, ApiError> {
    let repo = ClientRepository::new(state.db.pool.clone());
    let clients = repo.search(&query).await?;
    Ok(Json(clients))
}
