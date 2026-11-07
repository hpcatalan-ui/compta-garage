//! Authentication middleware

use axum::{
    extract::State,
    http::{Request, StatusCode},
    middleware::Next,
    response::Response,
};
use axum::body::Body;
use garage_compta_core::models::User;
use garage_compta_data::repositories::UserRepository;
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

use crate::state::AppState;

/// JWT claims
#[derive(Debug, Serialize, Deserialize)]
pub struct JwtClaims {
    pub sub: String, // User ID
    pub username: String,
    pub role: String,
    pub exp: usize, // Expiration timestamp
    pub iat: usize, // Issued at timestamp
}

impl JwtClaims {
    /// Creates new JWT claims
    pub fn new(user: &User, expiry_seconds: usize) -> Self {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as usize;
        
        Self {
            sub: user.id.to_string(),
            username: user.username.clone(),
            role: format!("{:?}", user.role),
            exp: now + expiry_seconds,
            iat: now,
        }
    }

    /// Checks if claims are expired
    pub fn is_expired(&self) -> bool {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as usize;
        
        self.exp < now
    }
}

/// Creates a JWT token for a user
pub fn create_jwt_token(
    user: &User,
    secret: &str,
    expiry_seconds: usize,
) -> Result<String, jsonwebtoken::errors::Error> {
    let claims = JwtClaims::new(user, expiry_seconds);
    
    encode(
        &Header::new(Algorithm::HS256),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
}

/// Verifies a JWT token
pub fn verify_jwt_token(
    token: &str,
    secret: &str,
) -> Result<JwtClaims, jsonwebtoken::errors::Error> {
    let mut validation = Validation::new(Algorithm::HS256);
    validation.validate_exp = true;
    
    decode::<JwtClaims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &validation,
    )
    .map(|data| data.claims)
}

/// Authentication middleware
pub async fn auth_middleware(
    State(state): State<AppState>,
    request: Request<Body>,
    next: Next,
) -> Result<Response<Body>, Response<Body>> {
    // Extract token from Authorization header
    let token = request.headers()
        .get("Authorization")
        .and_then(|h| h.to_str().ok())
        .and_then(|s| s.strip_prefix("Bearer "))
        .ok_or_else(|| {
            Response::builder()
                .status(StatusCode::UNAUTHORIZED)
                .body(Body::from("Missing or invalid Authorization header"))
                .unwrap()
        })?;
    
    // Verify the token
    let claims = verify_jwt_token(token, &state.config.jwt_secret)
        .map_err(|e| {
            Response::builder()
                .status(StatusCode::UNAUTHORIZED)
                .body(Body::from(format!("Invalid token: {}", e)))
                .unwrap()
        })?;
    
    // Check if token is expired
    if claims.is_expired() {
        return Err(Response::builder()
            .status(StatusCode::UNAUTHORIZED)
            .body(Body::from("Token expired"))
            .unwrap());
    }
    
    // Get user from database
    let user_repo = UserRepository::new(state.db.pool.clone());
    let user = user_repo.get(claims.sub.parse().unwrap())
        .await
        .map_err(|e| {
            Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(Body::from(format!("Failed to fetch user: {}", e)))
                .unwrap()
        })?
        .ok_or_else(|| {
            Response::builder()
                .status(StatusCode::UNAUTHORIZED)
                .body(Body::from("User not found"))
                .unwrap()
        })?;
    
    // Check if user is active
    if !user.is_active {
        return Err(Response::builder()
            .status(StatusCode::FORBIDDEN)
            .body(Body::from("User is inactive"))
            .unwrap());
    }
    
    // Check if user is locked
    if user.is_locked {
        return Err(Response::builder()
            .status(StatusCode::FORBIDDEN)
            .body(Body::from("User is locked"))
            .unwrap());
    }
    
    // Add user to request extensions
    let mut request = request;
    request.extensions_mut().insert(user);
    
    Ok(next.run(request).await)
}

/// Permission middleware
pub async fn permission_middleware(
    required_permission: garage_compta_core::models::Permission,
    State(_state): State<AppState>,
    request: Request<Body>,
    next: Next,
) -> Result<Response<Body>, Response<Body>> {
    // Get user from request extensions
    let user = request.extensions()
        .get::<User>()
        .ok_or_else(|| {
            Response::builder()
                .status(StatusCode::UNAUTHORIZED)
                .body(Body::from("User not authenticated"))
                .unwrap()
        })?;
    
    // Check permission - clone to avoid move issues
    if !user.has_permission(required_permission.clone()) {
        return Err(Response::builder()
            .status(StatusCode::FORBIDDEN)
            .body(Body::from(format!("Permission denied: {:?}", required_permission)))
            .unwrap());
    }
    
    Ok(next.run(request).await)
}
