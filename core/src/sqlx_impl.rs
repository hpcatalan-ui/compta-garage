//! SQLx trait implementations for all models

use sqlx::{sqlite::SqliteRow, Encode, Decode, FromRow, Type, Sqlite, Row};
use sqlx::encode::IsNull;
use chrono::{DateTime, Utc, NaiveDate, FixedOffset};
use rust_decimal::Decimal;
use uuid::Uuid;

use super::models::*;

// ============================================================================
// TYPE IMPLEMENTATIONS FOR CUSTOM TYPES
// ============================================================================

// Address - stored as pipe-delimited string: street|postal_code|city|country|additional
impl Type<Sqlite> for Address {
    fn type_info() -> sqlx::sqlite::SqliteTypeInfo {
        <String as Type<Sqlite>>::type_info()
    }
}

impl<'q> Encode<'q, Sqlite> for Address {
    fn encode_by_ref(&self, buf: &mut Vec<sqlx::sqlite::SqliteArgumentValue<'q>>) -> IsNull {
        let s = self.to_sql_string();
        <String as Encode<'q, Sqlite>>::encode(s, buf)
    }
}

impl<'r> Decode<'r, Sqlite> for Address {
    fn decode(value: <Sqlite as sqlx::database::HasValueRef<'r>>::ValueRef) -> Result<Self, sqlx::error::BoxDynError> {
        let s: String = <String as Decode<'r, Sqlite>>::decode(value)?;
        Self::from_sql_string(&s).ok_or_else(|| "Invalid Address format".into())
    }
}

// ============================================================================
// FROMROW IMPLEMENTATIONS FOR STRUCTS
// ============================================================================

// Address
impl<'r> FromRow<'r, SqliteRow> for Address {
    fn from_row(row: &'r SqliteRow) -> Result<Self, sqlx::Error> {
        Ok(Address {
            street: row.get("street"),
            postal_code: row.get("postal_code"),
            city: row.get("city"),
            country: row.get("country"),
            additional: row.get::<Option<String>, _>("additional"),
        })
    }
}

// Helper function to parse Decimal from string with default
fn parse_decimal(s: &str, default: Decimal) -> Decimal {
    s.parse().unwrap_or(default)
}

// Helper function to parse optional Decimal from string
fn parse_optional_decimal(s: Option<String>) -> Option<Decimal> {
    s.and_then(|s| s.parse().ok())
}

// Helper function to convert DateTime<FixedOffset> to DateTime<Utc>
fn to_utc(dt: Option<DateTime<FixedOffset>>) -> Option<DateTime<Utc>> {
    dt.map(|d| d.with_timezone(&Utc))
}

// Client
impl<'r> FromRow<'r, SqliteRow> for Client {
    fn from_row(row: &'r SqliteRow) -> Result<Self, sqlx::Error> {
        let balance_str: String = row.get("balance");
        let balance = parse_decimal(&balance_str, Decimal::ZERO);
        
        Ok(Client {
            id: row.get("id"),
            client_type: row.get("client_type"),
            name: row.get("name"),
            first_name: row.get::<Option<String>, _>("first_name"),
            company: row.get::<Option<String>, _>("company"),
            siret: row.get::<Option<String>, _>("siret"),
            tvat: row.get::<Option<String>, _>("tvat"),
            street: row.get("street"),
            postal_code: row.get("postal_code"),
            city: row.get("city"),
            country: row.get("country"),
            additional: row.get::<Option<String>, _>("additional"),
            phone: row.get("phone"),
            email: row.get::<Option<String>, _>("email"),
            is_active: row.get("is_active"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
            balance,
            payment_terms: row.get("payment_terms"),
            notes: row.get::<Option<String>, _>("notes"),
            code: row.get::<Option<String>, _>("code"),
        })
    }
}

// VehicleOwnerHistory
impl<'r> FromRow<'r, SqliteRow> for VehicleOwnerHistory {
    fn from_row(row: &'r SqliteRow) -> Result<Self, sqlx::Error> {
        let purchase_price_str: String = row.get("purchase_price");
        let purchase_price = parse_decimal(&purchase_price_str, Decimal::ZERO);
        
        Ok(VehicleOwnerHistory {
            id: row.get("id"),
            vehicle_id: row.get("vehicle_id"),
            client_id: row.get("client_id"),
            start_date: row.get("start_date"),
            end_date: row.get::<Option<DateTime<Utc>>, _>("end_date"),
            purchase_price,
            notes: row.get::<Option<String>, _>("notes"),
        })
    }
}

// Vehicle
impl<'r> FromRow<'r, SqliteRow> for Vehicle {
    fn from_row(row: &'r SqliteRow) -> Result<Self, sqlx::Error> {
        let next_inspection_date_str: Option<String> = row.get::<Option<String>, _>("next_inspection_date");
        let next_service_date_str: Option<String> = row.get::<Option<String>, _>("next_service_date");
        
        let next_inspection_date = next_inspection_date_str.and_then(|s| NaiveDate::parse_from_str(&s, "%Y-%m-%d").ok());
        let next_service_date = next_service_date_str.and_then(|s| NaiveDate::parse_from_str(&s, "%Y-%m-%d").ok());
        
        Ok(Vehicle {
            id: row.get("id"),
            registration: row.get("registration"),
            vin: row.get("vin"),
            make: row.get("make"),
            model: row.get("model"),
            version: row.get::<Option<String>, _>("version"),
            year: row.get("year"),
            mileage: row.get("mileage"),
            fuel_type: row.get("fuel_type"),
            transmission: row.get("transmission"),
            engine_capacity: row.get::<Option<i32>, _>("engine_capacity"),
            power: row.get::<Option<i32>, _>("power"),
            color: row.get::<Option<String>, _>("color"),
            current_owner_id: row.get("current_owner_id"),
            category: row.get("category"),
            next_inspection_date,
            next_service_date,
            notes: row.get::<Option<String>, _>("notes"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
            is_active: row.get("is_active"),
            code: row.get::<Option<String>, _>("code"),
        })
    }
}

// Supplier
impl<'r> FromRow<'r, SqliteRow> for Supplier {
    fn from_row(row: &'r SqliteRow) -> Result<Self, sqlx::Error> {
        Ok(Supplier {
            id: row.get("id"),
            name: row.get("name"),
            code: row.get::<Option<String>, _>("code"),
            street: row.get("street"),
            postal_code: row.get("postal_code"),
            city: row.get("city"),
            country: row.get("country"),
            additional: row.get::<Option<String>, _>("additional"),
            phone: row.get::<Option<String>, _>("phone"),
            email: row.get::<Option<String>, _>("email"),
            website: row.get::<Option<String>, _>("website"),
            payment_terms: row.get("payment_terms"),
            is_active: row.get("is_active"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
            notes: row.get::<Option<String>, _>("notes"),
            tvat: row.get::<Option<String>, _>("tvat"),
            siret: row.get::<Option<String>, _>("siret"),
            contact_name: row.get::<Option<String>, _>("contact_name"),
            contact_email: row.get::<Option<String>, _>("contact_email"),
            contact_phone: row.get::<Option<String>, _>("contact_phone"),
        })
    }
}

// Part
impl<'r> FromRow<'r, SqliteRow> for Part {
    fn from_row(row: &'r SqliteRow) -> Result<Self, sqlx::Error> {
        let purchase_price_str: String = row.get("purchase_price");
        let selling_price_str: String = row.get("selling_price");
        let tax_rate_str: String = row.get("tax_rate");
        let stock_quantity_str: String = row.get("stock_quantity");
        let minimum_stock_str: String = row.get("minimum_stock");
        let weight_str: Option<String> = row.get::<Option<String>, _>("weight");
        
        let purchase_price = parse_decimal(&purchase_price_str, Decimal::ZERO);
        let selling_price = parse_decimal(&selling_price_str, Decimal::ZERO);
        let tax_rate = parse_decimal(&tax_rate_str, Decimal::new(20, 2));
        let stock_quantity = parse_decimal(&stock_quantity_str, Decimal::ZERO);
        let minimum_stock = parse_decimal(&minimum_stock_str, Decimal::new(5, 0));
        let weight = parse_optional_decimal(weight_str);
        
        // Parse compatible_fuel_types from JSON string
        let compatible_fuel_types: Vec<FuelType> = row.get::<Option<String>, _>("compatible_fuel_types")
            .map(|s| serde_json::from_str(&s).unwrap_or_default())
            .unwrap_or_default();
        
        Ok(Part {
            id: row.get("id"),
            reference: row.get("reference"),
            supplier_reference: row.get::<Option<String>, _>("supplier_reference"),
            name: row.get("name"),
            description: row.get::<Option<String>, _>("description"),
            category: row.get("category"),
            supplier_id: row.get::<Option<Uuid>, _>("supplier_id"),
            purchase_price,
            selling_price,
            tax_rate,
            stock_quantity,
            minimum_stock,
            location: row.get::<Option<String>, _>("location"),
            is_active: row.get("is_active"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
            barcode: row.get::<Option<String>, _>("barcode"),
            unit: row.get("unit"),
            weight,
            dimensions: row.get::<Option<String>, _>("dimensions"),
            notes: row.get::<Option<String>, _>("notes"),
            compatible_vehicles: row.get::<Option<String>, _>("compatible_vehicles"),
            compatible_fuel_types,
            supplier: None,
        })
    }
}

// InvoiceLine
impl<'r> FromRow<'r, SqliteRow> for InvoiceLine {
    fn from_row(row: &'r SqliteRow) -> Result<Self, sqlx::Error> {
        let quantity_str: String = row.get("quantity");
        let unit_price_str: String = row.get("unit_price");
        let tax_rate_str: String = row.get("tax_rate");
        let discount_str: Option<String> = row.get::<Option<String>, _>("discount");
        let total_ht_str: String = row.get("total_ht");
        let total_ttc_str: String = row.get("total_ttc");
        
        let quantity = parse_decimal(&quantity_str, Decimal::ZERO);
        let unit_price = parse_decimal(&unit_price_str, Decimal::ZERO);
        let tax_rate = parse_decimal(&tax_rate_str, Decimal::new(20, 2));
        let discount = parse_optional_decimal(discount_str);
        let total_ht = parse_decimal(&total_ht_str, Decimal::ZERO);
        let total_ttc = parse_decimal(&total_ttc_str, Decimal::ZERO);
        
        Ok(InvoiceLine {
            id: row.get("id"),
            invoice_id: row.get("invoice_id"),
            line_number: row.get("line_number"),
            description: row.get("description"),
            quantity,
            unit_price,
            tax_rate,
            discount,
            total_ht,
            total_ttc,
            is_labor: row.get("is_labor"),
            part_id: row.get::<Option<Uuid>, _>("part_id"),
            labor_rate_id: row.get::<Option<Uuid>, _>("labor_rate_id"),
        })
    }
}

// Invoice
impl<'r> FromRow<'r, SqliteRow> for Invoice {
    fn from_row(row: &'r SqliteRow) -> Result<Self, sqlx::Error> {
        let subtotal_str: String = row.get("subtotal");
        let tax_amount_str: String = row.get("tax_amount");
        let total_str: String = row.get("total");
        let discount_str: Option<String> = row.get::<Option<String>, _>("discount");
        
        let subtotal = parse_decimal(&subtotal_str, Decimal::ZERO);
        let tax_amount = parse_decimal(&tax_amount_str, Decimal::ZERO);
        let total = parse_decimal(&total_str, Decimal::ZERO);
        let discount = parse_optional_decimal(discount_str);
        
        Ok(Invoice {
            id: row.get("id"),
            invoice_number: row.get("invoice_number"),
            invoice_type: row.get("invoice_type"),
            client_id: row.get("client_id"),
            vehicle_id: row.get::<Option<Uuid>, _>("vehicle_id"),
            date: row.get("date"),
            due_date: row.get("due_date"),
            status: row.get("status"),
            payment_method: row.get::<Option<PaymentMethod>, _>("payment_method"),
            payment_date: row.get::<Option<DateTime<Utc>>, _>("payment_date"),
            subtotal,
            tax_amount,
            total,
            discount,
            notes: row.get::<Option<String>, _>("notes"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
            created_by: row.get("created_by"),
            is_exported: row.get("is_exported"),
            exported_at: row.get::<Option<DateTime<Utc>>, _>("exported_at"),
            external_reference: row.get::<Option<String>, _>("external_reference"),
            client: None,
            vehicle: None,
        })
    }
}

// Payment
impl<'r> FromRow<'r, SqliteRow> for Payment {
    fn from_row(row: &'r SqliteRow) -> Result<Self, sqlx::Error> {
        let amount_str: String = row.get("amount");
        let amount = parse_decimal(&amount_str, Decimal::ZERO);
        
        Ok(Payment {
            id: row.get("id"),
            invoice_id: row.get("invoice_id"),
            amount,
            method: row.get("method"),
            date: row.get("date"),
            reference: row.get::<Option<String>, _>("reference"),
            notes: row.get::<Option<String>, _>("notes"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
            status: row.get("status"),
            invoice: None,
        })
    }
}

// User
impl<'r> FromRow<'r, SqliteRow> for User {
    fn from_row(row: &'r SqliteRow) -> Result<Self, sqlx::Error> {
        let last_login_str: Option<String> = row.get::<Option<String>, _>("last_login");
        let locked_at_str: Option<String> = row.get::<Option<String>, _>("locked_at");
        
        let last_login = to_utc(last_login_str.and_then(|s| DateTime::parse_from_rfc3339(&s).ok()));
        let locked_at = to_utc(locked_at_str.and_then(|s| DateTime::parse_from_rfc3339(&s).ok()));
        
        Ok(User {
            id: row.get("id"),
            username: row.get("username"),
            password_hash: row.get("password_hash"),
            role: row.get("role"),
            first_name: row.get::<Option<String>, _>("first_name"),
            last_name: row.get::<Option<String>, _>("last_name"),
            email: row.get::<Option<String>, _>("email"),
            phone: row.get::<Option<String>, _>("phone"),
            is_active: row.get("is_active"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
            last_login,
            failed_login_attempts: row.get("failed_login_attempts"),
            is_locked: row.get("is_locked"),
            locked_at,
            notes: row.get::<Option<String>, _>("notes"),
        })
    }
}
