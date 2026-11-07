//! Repository for User entities

use sqlx::sqlite::SqliteArgumentValue;
use uuid::Uuid;
use chrono::{DateTime, Utc};
use std::borrow::Cow;

use garage_compta_core::models::{User, UserRole};
use garage_compta_core::CoreError;
use crate::connection::DatabasePool;

/// User repository
#[derive(Debug, Clone)]
pub struct UserRepository {
    pool: DatabasePool,
}

impl UserRepository {
    /// Creates a new user repository
    pub fn new(pool: DatabasePool) -> Self {
        Self { pool }
    }

    /// Creates a new user
    pub async fn create(&self, user: &User) -> Result<User, CoreError> {
        let user = sqlx::query_as::<_, User>(
            r#"
            INSERT INTO users (
                id, username, password_hash, role, first_name, last_name,
                email, phone, is_active, created_at, updated_at,
                last_login, failed_login_attempts, is_locked, locked_at, notes
            ) VALUES (
                $1, $2, $3, $4, $5, $6,
                $7, $8, $9, $10, $11,
                $12, $13, $14, $15, $16
            )
            RETURNING *
            "#,
        )
        .bind(&user.id)
        .bind(&user.username)
        .bind(&user.password_hash)
        .bind(&user.role)
        .bind(&user.first_name)
        .bind(&user.last_name)
        .bind(&user.email)
        .bind(&user.phone)
        .bind(&user.is_active)
        .bind(&user.created_at)
        .bind(&user.updated_at)
        .bind(&user.last_login)
        .bind(&user.failed_login_attempts)
        .bind(&user.is_locked)
        .bind(&user.locked_at)
        .bind(&user.notes)
        .fetch_one(&self.pool)
        .await?;

        Ok(user)
    }

    /// Gets a user by ID
    pub async fn get(&self, id: Uuid) -> Result<Option<User>, CoreError> {
        let user = sqlx::query_as::<_, User>(
            "SELECT * FROM users WHERE id = $1 LIMIT 1"
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(user)
    }

    /// Gets a user by username
    pub async fn get_by_username(&self, username: &str) -> Result<Option<User>, CoreError> {
        let user = sqlx::query_as::<_, User>(
            "SELECT * FROM users WHERE username = $1 LIMIT 1"
        )
        .bind(username)
        .fetch_optional(&self.pool)
        .await?;

        Ok(user)
    }

    /// Gets all users
    pub async fn get_all(&self) -> Result<Vec<User>, CoreError> {
        let users = sqlx::query_as::<_, User>(
            "SELECT * FROM users ORDER BY username"
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(users)
    }

    /// Updates a user
    pub async fn update(&self, user: &User) -> Result<User, CoreError> {
        let user = sqlx::query_as::<_, User>(
            r#"
            UPDATE users SET
                username = $2, password_hash = $3, role = $4, first_name = $5, last_name = $6,
                email = $7, phone = $8, is_active = $9, updated_at = $10,
                last_login = $11, failed_login_attempts = $12, is_locked = $13, 
                locked_at = $14, notes = $15
            WHERE id = $1
            RETURNING *
            "#,
        )
        .bind(&user.id)
        .bind(&user.username)
        .bind(&user.password_hash)
        .bind(&user.role)
        .bind(&user.first_name)
        .bind(&user.last_name)
        .bind(&user.email)
        .bind(&user.phone)
        .bind(&user.is_active)
        .bind(&user.updated_at)
        .bind(&user.last_login)
        .bind(&user.failed_login_attempts)
        .bind(&user.is_locked)
        .bind(&user.locked_at)
        .bind(&user.notes)
        .bind(&user.id)
        .fetch_one(&self.pool)
        .await?;

        Ok(user)
    }

    /// Deletes a user
    pub async fn delete(&self, id: Uuid) -> Result<bool, CoreError> {
        let result = sqlx::query_scalar::<_, i64>(
            "DELETE FROM users WHERE id = $1"
        )
        .bind(id)
        .fetch_one(&self.pool)
        .await?;

        Ok(result > 0)
    }

    /// Updates user profile (without password)
    pub async fn update_profile(
        &self,
        id: Uuid,
        first_name: Option<String>,
        last_name: Option<String>,
        email: Option<String>,
        phone: Option<String>,
    ) -> Result<User, CoreError> {
        let user = sqlx::query_as::<_, User>(
            r#"
            UPDATE users SET
                first_name = COALESCE($2, first_name),
                last_name = COALESCE($3, last_name),
                email = COALESCE($4, email),
                phone = COALESCE($5, phone),
                updated_at = datetime('now')
            WHERE id = $1
            RETURNING *
            "#,
        )
        .bind(id)
        .bind(first_name)
        .bind(last_name)
        .bind(email)
        .bind(phone)
        .fetch_one(&self.pool)
        .await?;

        Ok(user)
    }

    /// Updates user password
    pub async fn update_password(&self, id: Uuid, new_password_hash: String) -> Result<User, CoreError> {
        let user = sqlx::query_as::<_, User>(
            "UPDATE users SET password_hash = $1, updated_at = datetime('now') WHERE id = $2 RETURNING *"
        )
        .bind(new_password_hash)
        .bind(id)
        .fetch_one(&self.pool)
        .await?;

        Ok(user)
    }

    /// Updates user role
    pub async fn update_role(&self, id: Uuid, role: UserRole) -> Result<User, CoreError> {
        let user = sqlx::query_as::<_, User>(
            "UPDATE users SET role = $1, updated_at = datetime('now') WHERE id = $2 RETURNING *"
        )
        .bind(&role)
        .bind(id)
        .fetch_one(&self.pool)
        .await?;

        Ok(user)
    }

    /// Locks a user account
    pub async fn lock(&self, id: Uuid) -> Result<User, CoreError> {
        let user = sqlx::query_as::<_, User>(
            "UPDATE users SET is_locked = true, locked_at = datetime('now'), failed_login_attempts = 0 WHERE id = $1 RETURNING *"
        )
        .bind(id)
        .fetch_one(&self.pool)
        .await?;

        Ok(user)
    }

    /// Unlocks a user account
    pub async fn unlock(&self, id: Uuid) -> Result<User, CoreError> {
        let user = sqlx::query_as::<_, User>(
            "UPDATE users SET is_locked = false, locked_at = NULL, failed_login_attempts = 0 WHERE id = $1 RETURNING *"
        )
        .bind(id)
        .fetch_one(&self.pool)
        .await?;

        Ok(user)
    }

    /// Increments failed login attempts
    pub async fn increment_failed_attempts(&self, id: Uuid) -> Result<User, CoreError> {
        let user = sqlx::query_as::<_, User>(
            "UPDATE users SET failed_login_attempts = failed_login_attempts + 1 WHERE id = $1 RETURNING *"
        )
        .bind(id)
        .fetch_one(&self.pool)
        .await?;

        Ok(user)
    }

    /// Resets failed login attempts
    pub async fn reset_failed_attempts(&self, id: Uuid) -> Result<User, CoreError> {
        let user = sqlx::query_as::<_, User>(
            "UPDATE users SET failed_login_attempts = 0 WHERE id = $1 RETURNING *"
        )
        .bind(id)
        .fetch_one(&self.pool)
        .await?;

        Ok(user)
    }

    /// Sets last login timestamp
    pub async fn set_last_login(&self, id: Uuid) -> Result<User, CoreError> {
        let user = sqlx::query_as::<_, User>(
            "UPDATE users SET last_login = datetime('now'), failed_login_attempts = 0 WHERE id = $1 RETURNING *"
        )
        .bind(id)
        .fetch_one(&self.pool)
        .await?;

        Ok(user)
    }

    /// Updates user partial fields
    pub async fn update_partial(
        &self,
        user_id: Uuid,
        last_login: Option<DateTime<Utc>>,
        failed_attempts: Option<i32>,
        is_locked: Option<bool>,
        locked_at: Option<DateTime<Utc>>,
    ) -> Result<(), CoreError> {
        let mut query = "UPDATE users SET updated_at = datetime('now')".to_string();
        let mut params: Vec<SqliteArgumentValue> = vec![];
        let mut param_index = 1;

        if let Some(last_login) = last_login {
            query = format!("{} last_login = ${}", query, param_index);
            params.push(SqliteArgumentValue::Text(Cow::Owned(last_login.to_rfc3339())));
            param_index += 1;
        }

        if let Some(failed_attempts) = failed_attempts {
            if param_index > 1 {
                query = format!("{}, ", query);
            }
            query = format!("{} failed_login_attempts = ${}", query, param_index);
            params.push(SqliteArgumentValue::Int64(failed_attempts as i64));
            param_index += 1;
        }

        if let Some(is_locked) = is_locked {
            if param_index > 1 {
                query = format!("{}, ", query);
            }
            query = format!("{} is_locked = ${}", query, param_index);
            params.push(SqliteArgumentValue::Int64(if is_locked { 1 } else { 0 }));
            param_index += 1;
        }

        if let Some(locked_at) = locked_at {
            if param_index > 1 {
                query = format!("{}, ", query);
            }
            query = format!("{} locked_at = ${}", query, param_index);
            params.push(SqliteArgumentValue::Text(Cow::Owned(locked_at.to_rfc3339())));
        }

        if param_index > 1 {
            query = format!("{}", query);
            params.push(SqliteArgumentValue::Text(Cow::Owned(user_id.to_string())));
            sqlx::query(&format!("{} WHERE id = ${}", query, param_index))
                .execute(&self.pool)
                .await?;
        }

        Ok(())
    }
}
