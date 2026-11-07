//! Repository for Client entities

use uuid::Uuid;

use garage_compta_core::models::{Client, ClientType};
use garage_compta_core::CoreError;
use crate::connection::DatabasePool;

/// Client repository
#[derive(Debug, Clone)]
pub struct ClientRepository {
    pool: DatabasePool,
}

impl ClientRepository {
    /// Creates a new client repository
    pub fn new(pool: DatabasePool) -> Self {
        Self { pool }
    }

    /// Creates a new client
    pub async fn create(&self, client: &Client) -> Result<Client, CoreError> {
        let client = sqlx::query_as::<_, Client>(
            r#"
            INSERT INTO clients (
                id, client_type, name, first_name, company, siret, tvat,
                street, postal_code, city, country, additional, phone, email,
                is_active, created_at, updated_at, balance, payment_terms, notes, code
            ) VALUES (
                $1, $2, $3, $4, $5, $6, $7,
                $8, $9, $10, $11, $12, $13, $14,
                $15, $16, $17, $18, $19, $20, $21
            )
            RETURNING *
            "#,
        )
        .bind(&client.id)
        .bind(&client.client_type)
        .bind(&client.name)
        .bind(&client.first_name)
        .bind(&client.company)
        .bind(&client.siret)
        .bind(&client.tvat)
        .bind(&client.street)
        .bind(&client.postal_code)
        .bind(&client.city)
        .bind(&client.country)
        .bind(&client.additional)
        .bind(&client.phone)
        .bind(&client.email)
        .bind(&client.is_active)
        .bind(&client.created_at)
        .bind(&client.updated_at)
        .bind(client.balance.to_string())
        .bind(&client.payment_terms)
        .bind(&client.notes)
        .bind(&client.code)
        .fetch_one(&self.pool)
        .await?;

        Ok(client)
    }

    /// Gets a client by ID
    pub async fn get(&self, id: Uuid) -> Result<Option<Client>, CoreError> {
        let client = sqlx::query_as::<_, Client>(
            r#"
            SELECT * FROM clients WHERE id = $1 LIMIT 1
            "#,
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(client)
    }

    /// Gets all clients
    pub async fn get_all(&self) -> Result<Vec<Client>, CoreError> {
        let clients = sqlx::query_as::<_, Client>(
            r#"
            SELECT * FROM clients ORDER BY name
            "#,
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(clients)
    }

    /// Updates a client
    pub async fn update(&self, client: &Client) -> Result<Client, CoreError> {
        let client = sqlx::query_as::<_, Client>(
            r#"
            UPDATE clients SET
                client_type = $2, name = $3, first_name = $4, company = $5,
                siret = $6, tvat = $7, street = $8, postal_code = $9,
                city = $10, country = $11, additional = $12, phone = $13,
                email = $14, is_active = $15, updated_at = $16,
                balance = $17, payment_terms = $18, notes = $19, code = $20
            WHERE id = $1
            RETURNING *
            "#,
        )
        .bind(&client.id)
        .bind(&client.client_type)
        .bind(&client.name)
        .bind(&client.first_name)
        .bind(&client.company)
        .bind(&client.siret)
        .bind(&client.tvat)
        .bind(&client.street)
        .bind(&client.postal_code)
        .bind(&client.city)
        .bind(&client.country)
        .bind(&client.additional)
        .bind(&client.phone)
        .bind(&client.email)
        .bind(&client.is_active)
        .bind(&client.updated_at)
        .bind(client.balance.to_string())
        .bind(&client.payment_terms)
        .bind(&client.notes)
        .bind(&client.code)
        .bind(&client.id)
        .fetch_one(&self.pool)
        .await?;

        Ok(client)
    }

    /// Deletes a client
    pub async fn delete(&self, id: Uuid) -> Result<bool, CoreError> {
        let result = sqlx::query_scalar::<_, i64>(
            "DELETE FROM clients WHERE id = $1"
        )
        .bind(id)
        .fetch_one(&self.pool)
        .await?;

        Ok(result > 0)
    }

    /// Gets clients by name (search)
    pub async fn search(&self, query: &str) -> Result<Vec<Client>, CoreError> {
        let clients = sqlx::query_as::<_, Client>(
            r#"
            SELECT * FROM clients 
            WHERE name LIKE $1 OR first_name LIKE $1 OR company LIKE $1
            ORDER BY name
            "#,
        )
        .bind(&format!("%{}%", query))
        .fetch_all(&self.pool)
        .await?;

        Ok(clients)
    }

    /// Gets clients by type
    pub async fn get_by_type(&self, client_type: ClientType) -> Result<Vec<Client>, CoreError> {
        let clients = sqlx::query_as::<_, Client>(
            r#"
            SELECT * FROM clients WHERE client_type = $1 ORDER BY name
            "#,
        )
        .bind(&client_type)
        .fetch_all(&self.pool)
        .await?;

        Ok(clients)
    }

    /// Gets active clients
    pub async fn get_active(&self) -> Result<Vec<Client>, CoreError> {
        let clients = sqlx::query_as::<_, Client>(
            "SELECT * FROM clients WHERE is_active = true ORDER BY name"
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(clients)
    }
}
