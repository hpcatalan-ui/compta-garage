//! Address model

use serde::{Deserialize, Serialize};

/// Address entity
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Address {
    /// Street address
    pub street: String,
    /// Postal code
    pub postal_code: String,
    /// City
    pub city: String,
    /// Country
    pub country: String,
    /// Additional address info (apartment, building, etc.)
    pub additional: Option<String>,
}

impl Address {
    /// Creates a new address
    pub fn new(street: String, postal_code: String, city: String, country: String) -> Self {
        Self {
            street,
            postal_code,
            city,
            country,
            additional: None,
        }
    }

    /// Full address as a single string
    pub fn to_string(&self) -> String {
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

    /// Pipe-delimited serialization for SQL storage
    pub fn to_sql_string(&self) -> String {
        let additional = self.additional.clone().unwrap_or_default();
        format!(
            "{}|{}|{}|{}|{}",
            self.street, self.postal_code, self.city, self.country, additional
        )
    }

    /// Deserialize from pipe-delimited string
    pub fn from_sql_string(s: &str) -> Option<Self> {
        let parts: Vec<&str> = s.split('|').collect();
        if parts.len() >= 4 {
            Some(Self {
                street: parts[0].to_string(),
                postal_code: parts[1].to_string(),
                city: parts[2].to_string(),
                country: parts[3].to_string(),
                additional: if parts.len() > 4 && !parts[4].is_empty() {
                    Some(parts[4].to_string())
                } else {
                    None
                },
            })
        } else {
            None
        }
    }
}

impl Default for Address {
    fn default() -> Self {
        Self {
            street: String::new(),
            postal_code: String::new(),
            city: String::new(),
            country: "France".to_string(),
            additional: None,
        }
    }
}
