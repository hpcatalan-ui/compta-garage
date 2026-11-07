//! Repository for Vehicle entities

use uuid::Uuid;
use chrono::Utc;

use garage_compta_core::models::{Vehicle, VehicleOwnerHistory};
use garage_compta_core::CoreError;
use crate::connection::DatabasePool;

/// Vehicle repository
#[derive(Debug, Clone)]
pub struct VehicleRepository {
    pool: DatabasePool,
}

impl VehicleRepository {
    /// Creates a new vehicle repository
    pub fn new(pool: DatabasePool) -> Self {
        Self { pool }
    }

    /// Creates a new vehicle
    pub async fn create(&self, vehicle: &Vehicle) -> Result<Vehicle, CoreError> {
        let next_inspection = vehicle.next_inspection_date.map(|d| d.to_string());
        let next_service = vehicle.next_service_date.map(|d| d.to_string());
        
        let vehicle = sqlx::query_as::<_, Vehicle>(
            r#"
            INSERT INTO vehicles (
                id, registration, vin, make, model, version, year, mileage,
                fuel_type, transmission, engine_capacity, power, color,
                current_owner_id, category, next_inspection_date, next_service_date,
                notes, created_at, updated_at, is_active, code
            ) VALUES (
                $1, $2, $3, $4, $5, $6, $7, $8,
                $9, $10, $11, $12, $13, $14, $15, $16, $17,
                $18, $19, $20, $21, $22
            )
            RETURNING *
            "#,
        )
        .bind(&vehicle.id)
        .bind(&vehicle.registration)
        .bind(&vehicle.vin)
        .bind(&vehicle.make)
        .bind(&vehicle.model)
        .bind(&vehicle.version)
        .bind(&vehicle.year)
        .bind(&vehicle.mileage)
        .bind(&vehicle.fuel_type)
        .bind(&vehicle.transmission)
        .bind(&vehicle.engine_capacity)
        .bind(&vehicle.power)
        .bind(&vehicle.color)
        .bind(&vehicle.current_owner_id)
        .bind(&vehicle.category)
        .bind(&next_inspection)
        .bind(&next_service)
        .bind(&vehicle.notes)
        .bind(&vehicle.created_at)
        .bind(&vehicle.updated_at)
        .bind(&vehicle.is_active)
        .bind(&vehicle.code)
        .fetch_one(&self.pool)
        .await?;

        Ok(vehicle)
    }

    /// Gets a vehicle by ID
    pub async fn get(&self, id: Uuid) -> Result<Option<Vehicle>, CoreError> {
        let vehicle = sqlx::query_as::<_, Vehicle>(
            "SELECT * FROM vehicles WHERE id = $1 LIMIT 1"
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(vehicle)
    }

    /// Gets all vehicles
    pub async fn get_all(&self) -> Result<Vec<Vehicle>, CoreError> {
        let vehicles = sqlx::query_as::<_, Vehicle>(
            "SELECT * FROM vehicles ORDER BY registration"
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(vehicles)
    }

    /// Updates a vehicle
    pub async fn update(&self, vehicle: &Vehicle) -> Result<Vehicle, CoreError> {
        let next_inspection = vehicle.next_inspection_date.map(|d| d.to_string());
        let next_service = vehicle.next_service_date.map(|d| d.to_string());
        
        let vehicle = sqlx::query_as::<_, Vehicle>(
            r#"
            UPDATE vehicles SET
                registration = $2, vin = $3, make = $4, model = $5, version = $6,
                year = $7, mileage = $8, fuel_type = $9, transmission = $10,
                engine_capacity = $11, power = $12, color = $13,
                current_owner_id = $14, category = $15, 
                next_inspection_date = $16, next_service_date = $17,
                notes = $18, updated_at = $19, is_active = $20, code = $21
            WHERE id = $1
            RETURNING *
            "#,
        )
        .bind(&vehicle.id)
        .bind(&vehicle.registration)
        .bind(&vehicle.vin)
        .bind(&vehicle.make)
        .bind(&vehicle.model)
        .bind(&vehicle.version)
        .bind(&vehicle.year)
        .bind(&vehicle.mileage)
        .bind(&vehicle.fuel_type)
        .bind(&vehicle.transmission)
        .bind(&vehicle.engine_capacity)
        .bind(&vehicle.power)
        .bind(&vehicle.color)
        .bind(&vehicle.current_owner_id)
        .bind(&vehicle.category)
        .bind(&next_inspection)
        .bind(&next_service)
        .bind(&vehicle.notes)
        .bind(&vehicle.updated_at)
        .bind(&vehicle.is_active)
        .bind(&vehicle.code)
        .bind(&vehicle.id)
        .fetch_one(&self.pool)
        .await?;

        Ok(vehicle)
    }

    /// Deletes a vehicle
    pub async fn delete(&self, id: Uuid) -> Result<bool, CoreError> {
        let result = sqlx::query_scalar::<_, i64>(
            "DELETE FROM vehicles WHERE id = $1"
        )
        .bind(id)
        .fetch_one(&self.pool)
        .await?;

        Ok(result > 0)
    }

    /// Gets vehicles by owner
    pub async fn get_by_owner(&self, owner_id: Uuid) -> Result<Vec<Vehicle>, CoreError> {
        let vehicles = sqlx::query_as::<_, Vehicle>(
            "SELECT * FROM vehicles WHERE current_owner_id = $1 ORDER BY registration"
        )
        .bind(owner_id)
        .fetch_all(&self.pool)
        .await?;

        Ok(vehicles)
    }

    /// Gets vehicles by registration
    pub async fn get_by_registration(&self, registration: &str) -> Result<Option<Vehicle>, CoreError> {
        let vehicle = sqlx::query_as::<_, Vehicle>(
            "SELECT * FROM vehicles WHERE registration = $1 LIMIT 1"
        )
        .bind(registration)
        .fetch_optional(&self.pool)
        .await?;

        Ok(vehicle)
    }

    /// Gets vehicles by VIN
    pub async fn get_by_vin(&self, vin: &str) -> Result<Option<Vehicle>, CoreError> {
        let vehicle = sqlx::query_as::<_, Vehicle>(
            "SELECT * FROM vehicles WHERE vin = $1 LIMIT 1"
        )
        .bind(vin)
        .fetch_optional(&self.pool)
        .await?;

        Ok(vehicle)
    }

    /// Changes vehicle owner
    pub async fn change_owner(
        &self,
        vehicle_id: Uuid,
        new_owner_id: Uuid,
        purchase_price: rust_decimal::Decimal,
        notes: Option<String>,
    ) -> Result<VehicleOwnerHistory, CoreError> {
        // Get current owner from vehicle
        let _vehicle = self.get(vehicle_id).await?.ok_or_else(|| {
            CoreError::ValidationError("Vehicle not found".to_string())
        })?;
        
        let now = Utc::now();
        
        // End previous ownership
        let _ = sqlx::query(
            "UPDATE vehicle_owner_history SET end_date = $1 WHERE vehicle_id = $2 AND end_date IS NULL"
        )
        .bind(now.to_string())
        .bind(vehicle_id)
        .execute(&self.pool)
        .await?;
        
        // Create new ownership record
        let history = sqlx::query_as::<_, VehicleOwnerHistory>(
            r#"
            INSERT INTO vehicle_owner_history (
                id, vehicle_id, client_id, start_date, purchase_price, notes
            ) VALUES ($1, $2, $3, $4, $5, $6)
            RETURNING *
            "#
        )
        .bind(Uuid::new_v4())
        .bind(vehicle_id)
        .bind(new_owner_id)
        .bind(now.to_string())
        .bind(purchase_price.to_string())
        .bind(notes)
        .fetch_one(&self.pool)
        .await?;
        
        // Update vehicle owner
        let _ = sqlx::query(
            "UPDATE vehicles SET current_owner_id = $1, updated_at = $2 WHERE id = $3"
        )
        .bind(new_owner_id)
        .bind(now.to_string())
        .bind(vehicle_id)
        .execute(&self.pool)
        .await?;
        
        Ok(history)
    }

    /// Gets ownership history for a vehicle
    pub async fn get_ownership_history(&self, vehicle_id: Uuid) -> Result<Vec<VehicleOwnerHistory>, CoreError> {
        let history = sqlx::query_as::<_, VehicleOwnerHistory>(
            "SELECT * FROM vehicle_owner_history WHERE vehicle_id = $1 ORDER BY start_date"
        )
        .bind(vehicle_id)
        .fetch_all(&self.pool)
        .await?;

        Ok(history)
    }
}
