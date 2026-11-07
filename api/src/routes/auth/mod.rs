//! Authentication routes

use axum::{Router, routing::post, Json, extract::State};
use axum::http::StatusCode;
use serde::{Serialize, Deserialize};

use garage_compta_data::repositories::UserRepository;
use crate::state::AppState;
use crate::middleware::{ApiError, auth::create_jwt_token};

/// Login request
#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

/// Login response
#[derive(Debug, Serialize)]
pub struct LoginResponse {
    pub token: String,
    pub user_id: String,
    pub username: String,
    pub role: String,
    pub expires_in: usize,
}

/// Creates the auth router
pub fn router(state: AppState) -> Router<AppState> {
    Router::new().with_state(state)
        .route("/login", post(login))
}

/// Login endpoint
async fn login(
    State(state): State<AppState>,
    Json(request): Json<LoginRequest>,
) -> Result<Json<LoginResponse>, ApiError> {
    // Get user from database
    let user_repo = UserRepository::new(state.db.pool.clone());
    let user = user_repo.get_by_username(&request.username).await?
        .ok_or_else(|| ApiError::new(StatusCode::UNAUTHORIZED, "Invalid credentials"))?;
    
    // Verify password (in a real app, use proper password hashing like argon2)
    // For now, we'll just check if the password matches (this is insecure!)
    // TODO: Implement proper password hashing
    if user.password_hash != request.password {
        // Increment failed attempts
        let _ = user_repo.increment_failed_attempts(user.id).await;
        return Err(ApiError::new(StatusCode::UNAUTHORIZED, "Invalid credentials"));
    }
    
    // Check if user is active
    if !user.is_active {
        return Err(ApiError::new(StatusCode::FORBIDDEN, "User is inactive"));
    }
    
    // Check if user is locked
    if user.is_locked {
        return Err(ApiError::new(StatusCode::FORBIDDEN, "User is locked"));
    }
    
    // Reset failed attempts
    let _ = user_repo.reset_failed_attempts(user.id).await;
    let _ = user_repo.set_last_login(user.id).await;
    
    // Create JWT token
    let token = create_jwt_token(&user, &state.config.jwt_secret, state.config.jwt_expiry as usize)
        .map_err(|e| ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, format!("Failed to create token: {}", e)))?;
    
    Ok(Json(LoginResponse {
        token,
        user_id: user.id.to_string(),
        username: user.username,
        role: format!("{:?}", user.role),
        expires_in: state.config.jwt_expiry as usize,
    }))
}
