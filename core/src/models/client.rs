//! Client model

use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Client type
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT", rename_all = "lowercase")]
pub enum ClientType {
    /// Individual client
    Individual,
    /// Professional/company client
    Professional,
}

impl Default for ClientType {
    fn default() -> Self {
        Self::Individual
    }
}

impl From<ClientType> for &str {
    fn from(val: ClientType) -> Self {
        match val {
            ClientType::Individual => "Individual",
            ClientType::Professional => "Professional",
        }
    }
}

impl std::str::FromStr for ClientType {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "individual" => Ok(Self::Individual),
            "professional" => Ok(Self::Professional),
            _ => Err(format!("Invalid ClientType: {}", s)),
        }
    }
}

/// Payment terms
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT", rename_all = "lowercase")]
pub enum PaymentTerms {
    /// Payment on receipt
    Immediate,
    /// Net 30 days
    Net30,
    /// Net 60 days
    Net60,
    /// Net 90 days
    Net90,
    /// End of month
    EndOfMonth,
}

impl Default for PaymentTerms {
    fn default() -> Self {
        Self::Net30
    }
}

impl From<PaymentTerms> for &str {
    fn from(val: PaymentTerms) -> Self {
        match val {
            PaymentTerms::Immediate => "Immediate",
            PaymentTerms::Net30 => "Net30",
            PaymentTerms::Net60 => "Net60",
            PaymentTerms::Net90 => "Net90",
            PaymentTerms::EndOfMonth => "EndOfMonth",
        }
    }
}

impl std::str::FromStr for PaymentTerms {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Immediate" => Ok(Self::Immediate),
            "Net30" => Ok(Self::Net30),
            "Net60" => Ok(Self::Net60),
            "Net90" => Ok(Self::Net90),
            "EndOfMonth" => Ok(Self::EndOfMonth),
            _ => Err(format!("Invalid PaymentTerms: {}", s)),
        }
    }
}

/// Client entity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Client {
    /// Unique identifier
    pub id: Uuid,
    /// Type of client
    pub client_type: ClientType,
    /// Name (for individuals: last name)
    pub name: String,
    /// First name (for individuals)
    pub first_name: Option<String>,
    /// Company name (for professionals)
    pub company: Option<String>,
    /// SIRET number (for French companies)
    pub siret: Option<String>,
    /// TVA intracommunautaire number
    pub tvat: Option<String>,
    /// Address
    pub street: String,
    pub postal_code: String,
    pub city: String,
    pub country: String,
    pub additional: Option<String>,
    /// Contact information
    pub phone: String,
    pub email: Option<String>,
    /// Status
    pub is_active: bool,
    /// Timestamps
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    /// Financial
    pub balance: Decimal,
    pub payment_terms: PaymentTerms,
    /// Additional info
    pub notes: Option<String>,
    pub code: Option<String>,
}

impl Client {
    /// Creates a new client
    pub fn new(
        name: String,
        street: String,
        postal_code: String,
        city: String,
        phone: String,
        client_type: ClientType,
    ) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            client_type,
            name,
            first_name: None,
            company: None,
            siret: None,
            tvat: None,
            street,
            postal_code,
            city,
            country: "France".to_string(),
            additional: None,
            phone,
            email: None,
            is_active: true,
            created_at: now,
            updated_at: now,
            balance: Decimal::ZERO,
            payment_terms: PaymentTerms::default(),
            notes: None,
            code: None,
        }
    }

    /// Full name (for individuals: first_name + name, for professionals: company or name)
    pub fn full_name(&self) -> String {
        match self.client_type {
            ClientType::Individual => {
                if let Some(ref first_name) = self.first_name {
                    format!("{} {}", first_name, self.name)
                } else {
                    self.name.clone()
                }
            }
            ClientType::Professional => self.company.clone().unwrap_or(self.name.clone()),
        }
    }

    /// Address as a single string
    pub fn address_string(&self) -> String {
        let mut parts = vec![
            self.street.clone(),
            self.postal_code.clone(),
            self.city.clone(),
        ];
        if let Some(ref additional) = self.additional {
            parts.insert(1, additional.clone());
        }
        parts.join(", ")
    }
}
