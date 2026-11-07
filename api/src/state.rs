//! Application state for the API

use std::sync::Arc;
use garage_compta_data::Database;

/// Application configuration
#[derive(Debug, Clone)]
pub struct AppConfig {
    /// Application name
    pub app_name: String,
    /// Application version
    pub app_version: String,
    /// Environment (development, production, test)
    pub environment: String,
    /// API port
    pub port: u16,
    /// JWT secret for authentication
    pub jwt_secret: String,
    /// JWT expiry in seconds
    pub jwt_expiry: i64,
    /// Database path
    pub database_path: String,
    /// Enable CORS
    pub enable_cors: bool,
    /// Allowed CORS origins
    pub cors_origins: Vec<String>,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            app_name: "Garage Compta".to_string(),
            app_version: "0.1.0".to_string(),
            environment: "development".to_string(),
            port: 3000,
            jwt_secret: "change-this-secret-in-production".to_string(),
            jwt_expiry: 86400, // 24 hours
            database_path: "./data/garage.db".to_string(),
            enable_cors: true,
            cors_origins: vec!["http://localhost:3000".to_string()],
        }
    }
}

/// Shared application state
#[derive(Debug, Clone)]
pub struct AppState {
    /// Database connection
    pub db: Arc<Database>,
    /// Configuration
    pub config: Arc<AppConfig>,
}

impl AppState {
    /// Creates a new application state
    pub fn new(db: Database, config: AppConfig) -> Self {
        Self {
            db: Arc::new(db),
            config: Arc::new(config),
        }
    }

    /// Creates a new application state with default configuration
    pub async fn new_with_defaults(db_path: &str) -> Result<Self, garage_compta_core::CoreError> {
        let config = AppConfig {
            database_path: db_path.to_string(),
            ..Default::default()
        };
        
        let db_config = garage_compta_data::DatabaseConfig::new(db_path);
        let db = Database::new(db_config).await?;
        
        Ok(Self::new(db, config))
    }
}
