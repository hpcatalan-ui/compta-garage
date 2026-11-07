//! Repository for Supplier entities

use uuid::Uuid;

use garage_compta_core::models::Supplier;
use garage_compta_core::CoreError;
use crate::connection::DatabasePool;

/// Supplier repository
#[derive(Debug, Clone)]
pub struct SupplierRepository {
    pool: DatabasePool,
}

impl SupplierRepository {
    /// Creates a new supplier repository
    pub fn new(pool: DatabasePool) -> Self {
        Self { pool }
    }

    /// Creates a new supplier
    pub async fn create(&self, supplier: &Supplier) -> Result<Supplier, CoreError> {
        let supplier = sqlx::query_as::<_, Supplier>(
            r#"
            INSERT INTO suppliers (
                id, name, code, street, postal_code, city, country, additional,
                phone, email, website, payment_terms, is_active, created_at, updated_at,
                notes, tvat, siret, contact_name, contact_email, contact_phone
            ) VALUES (
                $1, $2, $3, $4, $5, $6, $7, $8,
                $9, $10, $11, $12, $13, $14, $15,
                $16, $17, $18, $19, $20, $21
            )
            RETURNING *
            "#,
        )
        .bind(&supplier.id)
        .bind(&supplier.name)
        .bind(&supplier.code)
        .bind(&supplier.street)
        .bind(&supplier.postal_code)
        .bind(&supplier.city)
        .bind(&supplier.country)
        .bind(&supplier.additional)
        .bind(&supplier.phone)
        .bind(&supplier.email)
        .bind(&supplier.website)
        .bind(&supplier.payment_terms)
        .bind(&supplier.is_active)
        .bind(&supplier.created_at)
        .bind(&supplier.updated_at)
        .bind(&supplier.notes)
        .bind(&supplier.tvat)
        .bind(&supplier.siret)
        .bind(&supplier.contact_name)
        .bind(&supplier.contact_email)
        .bind(&supplier.contact_phone)
        .fetch_one(&self.pool)
        .await?;

        Ok(supplier)
    }

    /// Gets a supplier by ID
    pub async fn get(&self, id: Uuid) -> Result<Option<Supplier>, CoreError> {
        let supplier = sqlx::query_as::<_, Supplier>(
            "SELECT * FROM suppliers WHERE id = $1 LIMIT 1"
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(supplier)
    }

    /// Gets all suppliers
    pub async fn get_all(&self) -> Result<Vec<Supplier>, CoreError> {
        let suppliers = sqlx::query_as::<_, Supplier>(
            "SELECT * FROM suppliers ORDER BY name"
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(suppliers)
    }

    /// Updates a supplier
    pub async fn update(&self, supplier: &Supplier) -> Result<Supplier, CoreError> {
        let supplier = sqlx::query_as::<_, Supplier>(
            r#"
            UPDATE suppliers SET
                name = $2, code = $3, street = $4, postal_code = $5,
                city = $6, country = $7, additional = $8,
                phone = $9, email = $10, website = $11, payment_terms = $12,
                is_active = $13, updated_at = $14, notes = $15,
                tvat = $16, siret = $17, contact_name = $18, 
                contact_email = $19, contact_phone = $20
            WHERE id = $1
            RETURNING *
            "#,
        )
        .bind(&supplier.id)
        .bind(&supplier.name)
        .bind(&supplier.code)
        .bind(&supplier.street)
        .bind(&supplier.postal_code)
        .bind(&supplier.city)
        .bind(&supplier.country)
        .bind(&supplier.additional)
        .bind(&supplier.phone)
        .bind(&supplier.email)
        .bind(&supplier.website)
        .bind(&supplier.payment_terms)
        .bind(&supplier.is_active)
        .bind(&supplier.updated_at)
        .bind(&supplier.notes)
        .bind(&supplier.tvat)
        .bind(&supplier.siret)
        .bind(&supplier.contact_name)
        .bind(&supplier.contact_email)
        .bind(&supplier.contact_phone)
        .bind(&supplier.id)
        .fetch_one(&self.pool)
        .await?;

        Ok(supplier)
    }

    /// Deletes a supplier
    pub async fn delete(&self, id: Uuid) -> Result<bool, CoreError> {
        let result = sqlx::query_scalar::<_, i64>(
            "DELETE FROM suppliers WHERE id = $1"
        )
        .bind(id)
        .fetch_one(&self.pool)
        .await?;

        Ok(result > 0)
    }

    /// Gets suppliers by name
    pub async fn search(&self, query: &str) -> Result<Vec<Supplier>, CoreError> {
        let suppliers = sqlx::query_as::<_, Supplier>(
            r#"
            SELECT * FROM suppliers 
            WHERE name LIKE $1 OR code LIKE $1
            ORDER BY name
            "#,
        )
        .bind(&format!("%{}%", query))
        .fetch_all(&self.pool)
        .await?;

        Ok(suppliers)
    }

    /// Gets active suppliers
    pub async fn get_active(&self) -> Result<Vec<Supplier>, CoreError> {
        let suppliers = sqlx::query_as::<_, Supplier>(
            "SELECT * FROM suppliers WHERE is_active = true ORDER BY name"
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(suppliers)
    }
}
