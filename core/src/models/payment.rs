//! Payment model

use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::{Invoice, PaymentMethod};

/// Payment status
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT", rename_all = "lowercase")]
pub enum PaymentStatus {
    /// Payment is pending
    Pending,
    /// Payment completed
    Completed,
    /// Payment failed
    Failed,
    /// Payment refunded
    Refunded,
}

impl Default for PaymentStatus {
    fn default() -> Self {
        Self::Pending
    }
}

impl From<PaymentStatus> for &str {
    fn from(val: PaymentStatus) -> Self {
        match val {
            PaymentStatus::Pending => "Pending",
            PaymentStatus::Completed => "Completed",
            PaymentStatus::Failed => "Failed",
            PaymentStatus::Refunded => "Refunded",
        }
    }
}

impl std::str::FromStr for PaymentStatus {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Pending" => Ok(Self::Pending),
            "Completed" => Ok(Self::Completed),
            "Failed" => Ok(Self::Failed),
            "Refunded" => Ok(Self::Refunded),
            _ => Err(format!("Invalid PaymentStatus: {}", s)),
        }
    }
}

/// Payment entity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Payment {
    /// Unique identifier
    pub id: Uuid,
    /// Invoice ID
    pub invoice_id: Uuid,
    /// Amount
    pub amount: Decimal,
    /// Payment method
    pub method: PaymentMethod,
    /// Payment date
    pub date: DateTime<Utc>,
    /// Reference (check number, transaction ID, etc.)
    pub reference: Option<String>,
    /// Notes
    pub notes: Option<String>,
    /// Timestamps
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    /// Status
    pub status: PaymentStatus,
    // Navigation property
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice: Option<Invoice>,
}

impl Payment {
    /// Creates a new payment
    pub fn new(invoice_id: Uuid, amount: Decimal, method: PaymentMethod) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            invoice_id,
            amount,
            method,
            date: now,
            reference: None,
            notes: None,
            created_at: now,
            updated_at: now,
            status: PaymentStatus::Pending,
            invoice: None,
        }
    }

    /// Marks payment as completed
    pub fn complete(&mut self) {
        self.status = PaymentStatus::Completed;
        self.updated_at = Utc::now();
    }

    /// Marks payment as failed
    pub fn fail(&mut self) {
        self.status = PaymentStatus::Failed;
        self.updated_at = Utc::now();
    }
}
