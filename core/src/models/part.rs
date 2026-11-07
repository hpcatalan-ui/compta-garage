//! Part model

use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::{FuelType, Supplier};

/// Part category
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT", rename_all = "lowercase")]
pub enum PartCategory {
    Maintenance,
    Repair,
    Consumable,
    Accessory,
    Tire,
    Battery,
    Other,
}

impl Default for PartCategory {
    fn default() -> Self {
        Self::Maintenance
    }
}

impl From<PartCategory> for &str {
    fn from(val: PartCategory) -> Self {
        match val {
            PartCategory::Maintenance => "Maintenance",
            PartCategory::Repair => "Repair",
            PartCategory::Consumable => "Consumable",
            PartCategory::Accessory => "Accessory",
            PartCategory::Tire => "Tire",
            PartCategory::Battery => "Battery",
            PartCategory::Other => "Other",
        }
    }
}

impl std::str::FromStr for PartCategory {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Maintenance" => Ok(Self::Maintenance),
            "Repair" => Ok(Self::Repair),
            "Consumable" => Ok(Self::Consumable),
            "Accessory" => Ok(Self::Accessory),
            "Tire" => Ok(Self::Tire),
            "Battery" => Ok(Self::Battery),
            "Other" => Ok(Self::Other),
            _ => Err(format!("Invalid PartCategory: {}", s)),
        }
    }
}

/// Part unit
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT", rename_all = "lowercase")]
pub enum PartUnit {
    Unit,
    Liter,
    Kilogram,
    Meter,
    Set,
    Pair,
    Hour,
}

impl Default for PartUnit {
    fn default() -> Self {
        Self::Unit
    }
}

impl From<PartUnit> for &str {
    fn from(val: PartUnit) -> Self {
        match val {
            PartUnit::Unit => "Unit",
            PartUnit::Liter => "Liter",
            PartUnit::Kilogram => "Kilogram",
            PartUnit::Meter => "Meter",
            PartUnit::Set => "Set",
            PartUnit::Pair => "Pair",
            PartUnit::Hour => "Hour",
        }
    }
}

impl std::str::FromStr for PartUnit {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Unit" => Ok(Self::Unit),
            "Liter" => Ok(Self::Liter),
            "Kilogram" => Ok(Self::Kilogram),
            "Meter" => Ok(Self::Meter),
            "Set" => Ok(Self::Set),
            "Pair" => Ok(Self::Pair),
            "Hour" => Ok(Self::Hour),
            _ => Err(format!("Invalid PartUnit: {}", s)),
        }
    }
}

/// Part entity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Part {
    /// Unique identifier
    pub id: Uuid,
    /// Internal reference
    pub reference: String,
    /// Supplier reference
    pub supplier_reference: Option<String>,
    /// Name
    pub name: String,
    /// Description
    pub description: Option<String>,
    /// Category
    pub category: PartCategory,
    /// Supplier ID
    pub supplier_id: Option<Uuid>,
    /// Purchase price (HT)
    pub purchase_price: Decimal,
    /// Selling price (HT)
    pub selling_price: Decimal,
    /// Tax rate
    pub tax_rate: Decimal,
    /// Current stock quantity
    pub stock_quantity: Decimal,
    /// Minimum stock level
    pub minimum_stock: Decimal,
    /// Storage location
    pub location: Option<String>,
    /// Whether the part is active
    pub is_active: bool,
    /// Timestamps
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    /// Barcode
    pub barcode: Option<String>,
    /// Unit of measurement
    pub unit: PartUnit,
    /// Weight (kg)
    pub weight: Option<Decimal>,
    /// Dimensions
    pub dimensions: Option<String>,
    /// Notes
    pub notes: Option<String>,
    /// Compatible vehicles (JSON string)
    pub compatible_vehicles: Option<String>,
    /// Compatible fuel types (JSON string)
    pub compatible_fuel_types: Vec<FuelType>,
    // Navigation property
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supplier: Option<Supplier>,
}

impl Part {
    /// Creates a new part
    pub fn new(
        reference: String,
        name: String,
        purchase_price: Decimal,
        selling_price: Decimal,
    ) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            reference,
            supplier_reference: None,
            name,
            description: None,
            category: PartCategory::default(),
            supplier_id: None,
            purchase_price,
            selling_price,
            tax_rate: Decimal::new(20, 2), // 20% VAT by default
            stock_quantity: Decimal::ZERO,
            minimum_stock: Decimal::new(5, 0),
            location: None,
            is_active: true,
            created_at: now,
            updated_at: now,
            barcode: None,
            unit: PartUnit::default(),
            weight: None,
            dimensions: None,
            notes: None,
            compatible_vehicles: None,
            compatible_fuel_types: vec![],
            supplier: None,
        }
    }

    /// Calculates the margin percentage
    pub fn margin_percentage(&self) -> Decimal {
        if self.purchase_price == Decimal::ZERO {
            return Decimal::ZERO;
        }
        let margin = self.selling_price - self.purchase_price;
        (margin / self.purchase_price) * Decimal::new(100, 0)
    }

    /// Whether the part is below minimum stock
    pub fn is_below_minimum(&self) -> bool {
        self.stock_quantity < self.minimum_stock
    }

    /// Selling price with tax
    pub fn selling_price_ttc(&self) -> Decimal {
        self.selling_price * (Decimal::ONE + self.tax_rate / Decimal::new(100, 0))
    }
}
