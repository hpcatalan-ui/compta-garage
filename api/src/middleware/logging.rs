//! Logging middleware

use axum::{
    http::Request,
    middleware::Next,
    response::Response,
};
use axum::body::Body;
use tracing::{info, Span};
use std::time::Instant;
use uuid::Uuid;

/// Logging middleware for requests
pub async fn logging_middleware(request: Request<Body>, next: Next) -> Response<Body> {
    let method = request.method().clone();
    let uri = request.uri().clone();
    let request_id = Uuid::new_v4().to_string();
    
    // Create a span for tracing
    let span = Span::current();
    let _enter = span.enter();
    
    info!(
        request_id = %request_id,
        method = %method,
        uri = %uri,
        "Request started"
    );
    
    let start = Instant::now();
    
    let response = next.run(request).await;
    
    let duration = start.elapsed();
    let status = response.status();
    
    info!(
        request_id = %request_id,
        status = %status,
        duration = ?duration,
        "Request completed"
    );
    
    response
}
