//! Database migrations module

use std::fs;
use std::path::Path;

/// Runs all database migrations
pub async fn run_migrations(pool: &sqlx::SqlitePool) -> Result<(), sqlx::Error> {
    // Create _migrations tracking table if it doesn't exist
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS _migrations (
            version TEXT PRIMARY KEY,
            name TEXT NOT NULL,
            applied_at TEXT NOT NULL DEFAULT datetime('now')
        )
        "#,
    )
    .execute(pool)
    .await?;

    // Path to migrations directory
    let migrations_dir = Path::new("migrations");
    
    if !migrations_dir.exists() {
        return Ok(());
    }

    // Collect all SQL migration files
    let mut migration_files: Vec<_> = fs::read_dir(migrations_dir)?
        .filter_map(|entry| entry.ok())
        .filter(|entry| {
            let path = entry.path();
            path.extension().map(|ext| ext == "sql").unwrap_or(false)
        })
        .collect();
    
    // Sort by filename (alphabetical order)
    migration_files.sort_by_key(|entry| entry.file_name());

    // Apply each migration
    for entry in migration_files {
        let path = entry.path();
        let version = entry.file_name().to_string_lossy().into_owned();
        
        // Check if already applied
        let already_applied: Option<String> = sqlx::query_scalar(
            "SELECT version FROM _migrations WHERE version = $1"
        )
        .bind(&version)
        .fetch_optional(pool)
        .await?;
        
        if already_applied.is_some() {
            continue;
        }

        // Read and execute migration SQL
        let sql = fs::read_to_string(&path)?;
        sqlx::query(&sql).execute(pool).await?;
        
        // Record migration
        sqlx::query(
            "INSERT INTO _migrations (version, name) VALUES ($1, $2)"
        )
        .bind(&version)
        .bind(&version)
        .execute(pool)
        .await?;
    }
    
    Ok(())
}
