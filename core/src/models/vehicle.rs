//! Vehicle model

use chrono::{DateTime, NaiveDate, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Fuel type
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT", rename_all = "lowercase")]
pub enum FuelType {
    Petrol,
    Diesel,
    Electric,
    Hybrid,
    PlugInHybrid,
    LPG,
    CNG,
    Hydrogen,
}

impl Default for FuelType {
    fn default() -> Self {
        Self::Petrol
    }
}

impl From<FuelType> for &str {
    fn from(val: FuelType) -> Self {
        match val {
            FuelType::Petrol => "Petrol",
            FuelType::Diesel => "Diesel",
            FuelType::Electric => "Electric",
            FuelType::Hybrid => "Hybrid",
            FuelType::PlugInHybrid => "PlugInHybrid",
            FuelType::LPG => "LPG",
            FuelType::CNG => "CNG",
            FuelType::Hydrogen => "Hydrogen",
        }
    }
}

impl std::str::FromStr for FuelType {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Petrol" => Ok(Self::Petrol),
            "Diesel" => Ok(Self::Diesel),
            "Electric" => Ok(Self::Electric),
            "Hybrid" => Ok(Self::Hybrid),
            "PlugInHybrid" => Ok(Self::PlugInHybrid),
            "LPG" => Ok(Self::LPG),
            "CNG" => Ok(Self::CNG),
            "Hydrogen" => Ok(Self::Hydrogen),
            _ => Err(format!("Invalid FuelType: {}", s)),
        }
    }
}

/// Transmission type
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT", rename_all = "lowercase")]
pub enum Transmission {
    Manual,
    Automatic,
    SemiAutomatic,
    CVT,
}

impl Default for Transmission {
    fn default() -> Self {
        Self::Manual
    }
}

impl From<Transmission> for &str {
    fn from(val: Transmission) -> Self {
        match val {
            Transmission::Manual => "Manual",
            Transmission::Automatic => "Automatic",
            Transmission::SemiAutomatic => "SemiAutomatic",
            Transmission::CVT => "CVT",
        }
    }
}

impl std::str::FromStr for Transmission {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Manual" => Ok(Self::Manual),
            "Automatic" => Ok(Self::Automatic),
            "SemiAutomatic" => Ok(Self::SemiAutomatic),
            "CVT" => Ok(Self::CVT),
            _ => Err(format!("Invalid Transmission: {}", s)),
        }
    }
}

/// Vehicle category
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT", rename_all = "lowercase")]
pub enum VehicleCategory {
    Car,
    Motorcycle,
    Truck,
    Van,
    Bus,
    Trailer,
}

impl Default for VehicleCategory {
    fn default() -> Self {
        Self::Car
    }
}

impl From<VehicleCategory> for &str {
    fn from(val: VehicleCategory) -> Self {
        match val {
            VehicleCategory::Car => "Car",
            VehicleCategory::Motorcycle => "Motorcycle",
            VehicleCategory::Truck => "Truck",
            VehicleCategory::Van => "Van",
            VehicleCategory::Bus => "Bus",
            VehicleCategory::Trailer => "Trailer",
        }
    }
}

impl std::str::FromStr for VehicleCategory {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Car" => Ok(Self::Car),
            "Motorcycle" => Ok(Self::Motorcycle),
            "Truck" => Ok(Self::Truck),
            "Van" => Ok(Self::Van),
            "Bus" => Ok(Self::Bus),
            "Trailer" => Ok(Self::Trailer),
            _ => Err(format!("Invalid VehicleCategory: {}", s)),
        }
    }
}

/// Vehicle entity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Vehicle {
    /// Unique identifier
    pub id: Uuid,
    /// Registration plate (French: immatriculation)
    pub registration: String,
    /// Vehicle Identification Number
    pub vin: String,
    /// Manufacturer
    pub make: String,
    /// Model
    pub model: String,
    /// Version
    pub version: Option<String>,
    /// Year
    pub year: i32,
    /// Current mileage (km)
    pub mileage: i32,
    /// Fuel type
    pub fuel_type: FuelType,
    /// Transmission
    pub transmission: Transmission,
    /// Engine capacity (cc)
    pub engine_capacity: Option<i32>,
    /// Power (hp)
    pub power: Option<i32>,
    /// Color
    pub color: Option<String>,
    /// Current owner ID
    pub current_owner_id: Uuid,
    /// Category
    pub category: VehicleCategory,
    /// Next inspection date
    pub next_inspection_date: Option<NaiveDate>,
    /// Next service date
    pub next_service_date: Option<NaiveDate>,
    /// Notes
    pub notes: Option<String>,
    /// Timestamps
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    /// Status
    pub is_active: bool,
    /// Internal code
    pub code: Option<String>,
}

impl Vehicle {
    /// Creates a new vehicle
    pub fn new(
        registration: String,
        vin: String,
        make: String,
        model: String,
        year: i32,
        current_owner_id: Uuid,
    ) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            registration,
            vin,
            make,
            model,
            version: None,
            year,
            mileage: 0,
            fuel_type: FuelType::default(),
            transmission: Transmission::default(),
            engine_capacity: None,
            power: None,
            color: None,
            current_owner_id,
            category: VehicleCategory::default(),
            next_inspection_date: None,
            next_service_date: None,
            notes: None,
            created_at: now,
            updated_at: now,
            is_active: true,
            code: None,
        }
    }

    /// Display name
    pub fn display_name(&self) -> String {
        if let Some(ref version) = self.version {
            format!("{} {} {}", self.make, self.model, version)
        } else {
            format!("{} {}", self.make, self.model)
        }
    }
}

/// Vehicle owner history entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VehicleOwnerHistory {
    /// Unique identifier
    pub id: Uuid,
    /// Vehicle ID
    pub vehicle_id: Uuid,
    /// Client ID
    pub client_id: Uuid,
    /// Start date of ownership
    pub start_date: DateTime<Utc>,
    /// End date of ownership (None if current owner)
    pub end_date: Option<DateTime<Utc>>,
    /// Purchase price
    pub purchase_price: Decimal,
    /// Notes
    pub notes: Option<String>,
}

impl VehicleOwnerHistory {
    /// Creates a new owner history entry
    pub fn new(vehicle_id: Uuid, client_id: Uuid, start_date: DateTime<Utc>, purchase_price: Decimal) -> Self {
        Self {
            id: Uuid::new_v4(),
            vehicle_id,
            client_id,
            start_date,
            end_date: None,
            purchase_price,
            notes: None,
        }
    }

    /// Marks this as ended ownership
    pub fn end_ownership(&mut self, end_date: DateTime<Utc>) {
        self.end_date = Some(end_date);
    }
}
