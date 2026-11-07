//! Middleware for error handling

use axum::{
    http::Request,
    middleware::Next,
    response::{IntoResponse, Response},
    Json,
};
use axum::body::Body;
use serde_json::json;
use garage_compta_core::CoreError;

/// API error type
#[derive(Debug)]
pub struct ApiError {
    pub status_code: axum::http::StatusCode,
    pub message: String,
    pub details: Option<String>,
}

impl ApiError {
    /// Creates a new API error
    pub fn new(status_code: axum::http::StatusCode, message: impl Into<String>) -> Self {
        Self {
            status_code,
            message: message.into(),
            details: None,
        }
    }

    /// Creates an error with details
    pub fn with_details(
        status_code: axum::http::StatusCode,
        message: impl Into<String>,
        details: impl Into<String>,
    ) -> Self {
        Self {
            status_code,
            message: message.into(),
            details: Some(details.into()),
        }
    }
}

impl From<CoreError> for ApiError {
    fn from(error: CoreError) -> Self {
        match error {
            CoreError::DatabaseError(_) => ApiError::new(axum::http::StatusCode::INTERNAL_SERVER_ERROR, "Database error"),
            CoreError::ValidationError(msg) => ApiError::new(axum::http::StatusCode::BAD_REQUEST, msg),
            CoreError::InvoiceNumberingError(msg) => ApiError::new(axum::http::StatusCode::INTERNAL_SERVER_ERROR, msg),
            CoreError::ImportError(msg) => ApiError::new(axum::http::StatusCode::BAD_REQUEST, msg),
            CoreError::ExportError(msg) => ApiError::new(axum::http::StatusCode::INTERNAL_SERVER_ERROR, msg),
            CoreError::CalculationError(msg) => ApiError::new(axum::http::StatusCode::INTERNAL_SERVER_ERROR, msg),
            CoreError::ConversionError(msg) => ApiError::new(axum::http::StatusCode::BAD_REQUEST, msg),
            CoreError::IoError(_) => ApiError::new(axum::http::StatusCode::INTERNAL_SERVER_ERROR, "IO error"),
            CoreError::CsvError(_) => ApiError::new(axum::http::StatusCode::BAD_REQUEST, "CSV parsing error"),
            CoreError::XlsError(msg) => ApiError::new(axum::http::StatusCode::BAD_REQUEST, msg),
            CoreError::LockError(msg) => ApiError::new(axum::http::StatusCode::INTERNAL_SERVER_ERROR, msg),
            CoreError::SqlxError(_) => ApiError::new(axum::http::StatusCode::INTERNAL_SERVER_ERROR, "Database error"),
        }
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response<Body> {
        let mut response = Json(json!({
            "error": self.message,
            "status": self.status_code.as_u16(),
        }));
        
        if let Some(details) = self.details {
            response = Json(json!({
                "error": self.message,
                "status": self.status_code.as_u16(),
                "details": details,
            }));
        }
        
        (self.status_code, response).into_response()
    }
}

/// Error handler middleware
pub async fn error_handler_middleware(
    request: Request<Body>,
    next: Next,
) -> Response<Body> {
    let response = next.run(request).await;
    
    // If the response is a server error, convert it to ApiError response
    if response.status().is_server_error() {
        let status = response.status();
        let body = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap_or_default();
        
        let message = String::from_utf8_lossy(&body).to_string();
        let api_error = ApiError::new(status, message);
        return api_error.into_response();
    }
    
    response
}
