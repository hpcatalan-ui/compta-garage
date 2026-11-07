//! Repository for Part entities

use uuid::Uuid;
use serde_json;

use garage_compta_core::models::{Part, PartCategory};
use garage_compta_core::CoreError;
use crate::connection::DatabasePool;

/// Part repository
#[derive(Debug, Clone)]
pub struct PartRepository {
    pool: DatabasePool,
}

impl PartRepository {
    /// Creates a new part repository
    pub fn new(pool: DatabasePool) -> Self {
        Self { pool }
    }

    /// Creates a new part
    pub async fn create(&self, part: &Part) -> Result<Part, CoreError> {
        // Serialize compatible_fuel_types to JSON string
        let compatible_fuel_types = serde_json::to_string(&part.compatible_fuel_types)
            .unwrap_or_default();
        
        let part = sqlx::query_as::<_, Part>(
            r#"
            INSERT INTO parts (
                id, reference, supplier_reference, name, description,
                category, supplier_id, purchase_price, selling_price, tax_rate,
                stock_quantity, minimum_stock, location, is_active, created_at, updated_at,
                barcode, unit, weight, dimensions, notes, 
                compatible_vehicles, compatible_fuel_types
            ) VALUES (
                $1, $2, $3, $4, $5,
                $6, $7, $8, $9, $10,
                $11, $12, $13, $14, $15, $16,
                $17, $18, $19, $20, $21,
                $22, $23
            )
            RETURNING *
            "#,
        )
        .bind(&part.id)
        .bind(&part.reference)
        .bind(&part.supplier_reference)
        .bind(&part.name)
        .bind(&part.description)
        .bind(&part.category)
        .bind(&part.supplier_id)
        .bind(part.purchase_price.to_string())
        .bind(part.selling_price.to_string())
        .bind(part.tax_rate.to_string())
        .bind(part.stock_quantity.to_string())
        .bind(part.minimum_stock.to_string())
        .bind(&part.location)
        .bind(&part.is_active)
        .bind(&part.created_at)
        .bind(&part.updated_at)
        .bind(&part.barcode)
        .bind(&part.unit)
        .bind(part.weight.map(|w| w.to_string()))
        .bind(&part.dimensions)
        .bind(&part.notes)
        .bind(&part.compatible_vehicles)
        .bind(&compatible_fuel_types)
        .fetch_one(&self.pool)
        .await?;

        Ok(part)
    }

    /// Gets a part by ID
    pub async fn get(&self, id: Uuid) -> Result<Option<Part>, CoreError> {
        let part = sqlx::query_as::<_, Part>(
            "SELECT * FROM parts WHERE id = $1 LIMIT 1"
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(part)
    }

    /// Gets all parts
    pub async fn get_all(&self) -> Result<Vec<Part>, CoreError> {
        let parts = sqlx::query_as::<_, Part>(
            "SELECT * FROM parts ORDER BY name"
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(parts)
    }

    /// Updates a part
    pub async fn update(&self, part: &Part) -> Result<Part, CoreError> {
        // Serialize compatible_fuel_types to JSON string
        let compatible_fuel_types = serde_json::to_string(&part.compatible_fuel_types)
            .unwrap_or_default();
        
        let part = sqlx::query_as::<_, Part>(
            r#"
            UPDATE parts SET
                reference = $2, supplier_reference = $3, name = $4, description = $5,
                category = $6, supplier_id = $7, purchase_price = $8, selling_price = $9,
                tax_rate = $10, stock_quantity = $11, minimum_stock = $12, 
                location = $13, is_active = $14, updated_at = $15,
                barcode = $16, unit = $17, weight = $18, dimensions = $19, notes = $20,
                compatible_vehicles = $21, compatible_fuel_types = $22
            WHERE id = $1
            RETURNING *
            "#,
        )
        .bind(&part.id)
        .bind(&part.reference)
        .bind(&part.supplier_reference)
        .bind(&part.name)
        .bind(&part.description)
        .bind(&part.category)
        .bind(&part.supplier_id)
        .bind(part.purchase_price.to_string())
        .bind(part.selling_price.to_string())
        .bind(part.tax_rate.to_string())
        .bind(part.stock_quantity.to_string())
        .bind(part.minimum_stock.to_string())
        .bind(&part.location)
        .bind(&part.is_active)
        .bind(&part.updated_at)
        .bind(&part.barcode)
        .bind(&part.unit)
        .bind(part.weight.map(|w| w.to_string()))
        .bind(&part.dimensions)
        .bind(&part.notes)
        .bind(&part.compatible_vehicles)
        .bind(&compatible_fuel_types)
        .bind(&part.id)
        .fetch_one(&self.pool)
        .await?;

        Ok(part)
    }

    /// Deletes a part
    pub async fn delete(&self, id: Uuid) -> Result<bool, CoreError> {
        let result = sqlx::query_scalar::<_, i64>(
            "DELETE FROM parts WHERE id = $1"
        )
        .bind(id)
        .fetch_one(&self.pool)
        .await?;

        Ok(result > 0)
    }

    /// Gets parts by category
    pub async fn get_by_category(&self, category: PartCategory) -> Result<Vec<Part>, CoreError> {
        let parts = sqlx::query_as::<_, Part>(
            "SELECT * FROM parts WHERE category = $1 ORDER BY name"
        )
        .bind(&category)
        .fetch_all(&self.pool)
        .await?;

        Ok(parts)
    }

    /// Gets parts by reference
    pub async fn get_by_reference(&self, reference: &str) -> Result<Option<Part>, CoreError> {
        let part = sqlx::query_as::<_, Part>(
            "SELECT * FROM parts WHERE reference = $1 LIMIT 1"
        )
        .bind(reference)
        .fetch_optional(&self.pool)
        .await?;

        Ok(part)
    }

    /// Gets parts below minimum stock
    pub async fn get_below_minimum(&self) -> Result<Vec<Part>, CoreError> {
        let parts = sqlx::query_as::<_, Part>(
            r#"
            SELECT * FROM parts 
            WHERE stock_quantity < minimum_stock AND is_active = true
            ORDER BY name
            "#
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(parts)
    }

    /// Updates stock quantity
    pub async fn update_stock(&self, id: Uuid, quantity: rust_decimal::Decimal) -> Result<Part, CoreError> {
        let part = sqlx::query_as::<_, Part>(
            "UPDATE parts SET stock_quantity = $1, updated_at = datetime('now') WHERE id = $2 RETURNING *"
        )
        .bind(quantity.to_string())
        .bind(id)
        .fetch_one(&self.pool)
        .await?;

        Ok(part)
    }

    /// Adjusts stock quantity (positive = add, negative = remove)
    pub async fn adjust_stock(&self, id: Uuid, adjustment: rust_decimal::Decimal) -> Result<Part, CoreError> {
        let part = self.get(id).await?.ok_or_else(|| {
            CoreError::ValidationError("Part not found".to_string())
        })?;
        
        let new_quantity = part.stock_quantity + adjustment;
        self.update_stock(id, new_quantity).await
    }

    /// Searches parts
    pub async fn search(&self, query: &str) -> Result<Vec<Part>, CoreError> {
        let parts = sqlx::query_as::<_, Part>(
            r#"
            SELECT * FROM parts 
            WHERE name LIKE $1 OR reference LIKE $1 OR description LIKE $1
            ORDER BY name
            "#,
        )
        .bind(&format!("%{}%", query))
        .fetch_all(&self.pool)
        .await?;

        Ok(parts)
    }
}
