//! Supplier model

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Supplier entity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Supplier {
    /// Unique identifier
    pub id: Uuid,
    /// Name
    pub name: String,
    /// Code
    pub code: Option<String>,
    /// Address
    pub street: String,
    pub postal_code: String,
    pub city: String,
    pub country: String,
    pub additional: Option<String>,
    /// Contact information
    pub phone: Option<String>,
    pub email: Option<String>,
    pub website: Option<String>,
    /// Payment terms
    pub payment_terms: String,
    /// Whether the supplier is active
    pub is_active: bool,
    /// Timestamps
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    /// Notes
    pub notes: Option<String>,
    /// TVA intracommunautaire number
    pub tvat: Option<String>,
    /// SIRET number (for French suppliers)
    pub siret: Option<String>,
    /// Contact person
    pub contact_name: Option<String>,
    pub contact_email: Option<String>,
    pub contact_phone: Option<String>,
}

impl Supplier {
    /// Creates a new supplier
    pub fn new(name: String, street: String, postal_code: String, city: String) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            name,
            code: None,
            street,
            postal_code,
            city,
            country: "France".to_string(),
            additional: None,
            phone: None,
            email: None,
            website: None,
            payment_terms: "Net30".to_string(),
            is_active: true,
            created_at: now,
            updated_at: now,
            notes: None,
            tvat: None,
            siret: None,
            contact_name: None,
            contact_email: None,
            contact_phone: None,
        }
    }

    /// Full address
    pub fn address_string(&self) -> String {
        let mut parts = vec![
            self.street.clone(),
            self.postal_code.clone(),
            self.city.clone(),
            self.country.clone(),
        ];
        if let Some(ref additional) = self.additional {
            parts.insert(1, additional.clone());
        }
        parts.join(", ")
    }
}
