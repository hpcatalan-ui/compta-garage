//! Database connection management for SQLite

use sqlx::sqlite::{SqliteConnectOptions, SqlitePool, SqlitePoolOptions};
use std::path::Path;

use garage_compta_core::CoreError;

/// Database configuration
#[derive(Debug, Clone)]
pub struct DatabaseConfig {
    /// Path to the SQLite database file
    pub path: String,
    /// Enable WAL mode for better performance
    pub enable_wal: bool,
    /// Enable foreign keys
    pub enable_foreign_keys: bool,
}

impl Default for DatabaseConfig {
    fn default() -> Self {
        Self {
            path: "./data/garage.db".to_string(),
            enable_wal: true,
            enable_foreign_keys: true,
        }
    }
}

impl DatabaseConfig {
    /// Creates a new configuration from a path
    pub fn new(path: impl AsRef<Path>) -> Self {
        Self {
            path: path.as_ref().to_string_lossy().into_owned(),
            ..Default::default()
        }
    }
}

/// Database connection pool type
pub type DatabasePool = SqlitePool;

/// Database context (pool + configuration)
#[derive(Debug, Clone)]
pub struct Database {
    pub pool: DatabasePool,
    pub config: DatabaseConfig,
}

impl Database {
    /// Creates a new database connection
    pub async fn new(config: DatabaseConfig) -> Result<Self, CoreError> {
        // Create data directory if it doesn't exist
        let path = Path::new(&config.path);
        if let Some(parent) = path.parent() {
            tokio::fs::create_dir_all(parent)
                .await
                .map_err(|e| CoreError::IoError(e.to_string()))?;
        }

        // Connection options
        let mut connect_options = SqliteConnectOptions::new()
            .filename(&config.path)
            .create_if_missing(true);

        // Enable WAL mode for better performance
        if config.enable_wal {
            connect_options = connect_options.journal_mode(sqlx::sqlite::SqliteJournalMode::Wal);
        }

        // Create connection pool
        let pool = SqlitePoolOptions::new()
            .max_connections(5)
            .connect_with(connect_options)
            .await
            .map_err(|e| CoreError::SqlxError(e))?;

        // Enable foreign keys
        if config.enable_foreign_keys {
            sqlx::query("PRAGMA foreign_keys = ON")
                .execute(&pool)
                .await
                .map_err(|e| CoreError::SqlxError(e))?;
        }

        Ok(Self { pool, config })
    }

    /// Runs database migrations
    pub async fn migrate(&self) -> Result<(), CoreError> {
        crate::migrations::run_migrations(&self.pool)
            .await
            .map_err(|e| CoreError::DatabaseError(e.to_string()))?;
        
        Ok(())
    }

    /// Checks if the database file exists
    pub fn exists(&self) -> bool {
        Path::new(&self.config.path).exists()
    }
}
