//! Invoice model

use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::{Client, Vehicle};

/// Invoice type
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT", rename_all = "lowercase")]
pub enum InvoiceType {
    /// Standard invoice
    Invoice,
    /// Credit note
    CreditNote,
    /// Proforma invoice
    Proforma,
    /// Quote/Estimate
    Quote,
}

impl Default for InvoiceType {
    fn default() -> Self {
        Self::Invoice
    }
}

impl From<InvoiceType> for &str {
    fn from(val: InvoiceType) -> Self {
        match val {
            InvoiceType::Invoice => "Invoice",
            InvoiceType::CreditNote => "CreditNote",
            InvoiceType::Proforma => "Proforma",
            InvoiceType::Quote => "Quote",
        }
    }
}

impl std::str::FromStr for InvoiceType {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Invoice" => Ok(Self::Invoice),
            "CreditNote" => Ok(Self::CreditNote),
            "Proforma" => Ok(Self::Proforma),
            "Quote" => Ok(Self::Quote),
            _ => Err(format!("Invalid InvoiceType: {}", s)),
        }
    }
}

/// Invoice status
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT", rename_all = "lowercase")]
pub enum InvoiceStatus {
    /// Draft, not yet finalized
    Draft,
    /// Validated and ready to be sent
    Validated,
    /// Sent to client
    Sent,
    /// Partially paid
    Partial,
    /// Fully paid
    Paid,
    /// Cancelled
    Cancelled,
}

impl Default for InvoiceStatus {
    fn default() -> Self {
        Self::Draft
    }
}

impl From<InvoiceStatus> for &str {
    fn from(val: InvoiceStatus) -> Self {
        match val {
            InvoiceStatus::Draft => "Draft",
            InvoiceStatus::Validated => "Validated",
            InvoiceStatus::Sent => "Sent",
            InvoiceStatus::Partial => "Partial",
            InvoiceStatus::Paid => "Paid",
            InvoiceStatus::Cancelled => "Cancelled",
        }
    }
}

impl std::str::FromStr for InvoiceStatus {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Draft" => Ok(Self::Draft),
            "Validated" => Ok(Self::Validated),
            "Sent" => Ok(Self::Sent),
            "Partial" => Ok(Self::Partial),
            "Paid" => Ok(Self::Paid),
            "Cancelled" => Ok(Self::Cancelled),
            _ => Err(format!("Invalid InvoiceStatus: {}", s)),
        }
    }
}

/// Payment method
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT", rename_all = "lowercase")]
pub enum PaymentMethod {
    Cash,
    Check,
    BankTransfer,
    CreditCard,
    DebitCard,
    PayPal,
    Other,
}

impl Default for PaymentMethod {
    fn default() -> Self {
        Self::Cash
    }
}

impl From<PaymentMethod> for &str {
    fn from(val: PaymentMethod) -> Self {
        match val {
            PaymentMethod::Cash => "Cash",
            PaymentMethod::Check => "Check",
            PaymentMethod::BankTransfer => "BankTransfer",
            PaymentMethod::CreditCard => "CreditCard",
            PaymentMethod::DebitCard => "DebitCard",
            PaymentMethod::PayPal => "PayPal",
            PaymentMethod::Other => "Other",
        }
    }
}

impl std::str::FromStr for PaymentMethod {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Cash" => Ok(Self::Cash),
            "Check" => Ok(Self::Check),
            "BankTransfer" => Ok(Self::BankTransfer),
            "CreditCard" => Ok(Self::CreditCard),
            "DebitCard" => Ok(Self::DebitCard),
            "PayPal" => Ok(Self::PayPal),
            "Other" => Ok(Self::Other),
            _ => Err(format!("Invalid PaymentMethod: {}", s)),
        }
    }
}

/// Invoice line entity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InvoiceLine {
    /// Unique identifier
    pub id: Uuid,
    /// Invoice ID
    pub invoice_id: Uuid,
    /// Line number
    pub line_number: i32,
    /// Description
    pub description: String,
    /// Quantity
    pub quantity: Decimal,
    /// Unit price (HT)
    pub unit_price: Decimal,
    /// Tax rate
    pub tax_rate: Decimal,
    /// Discount rate
    pub discount: Option<Decimal>,
    /// Total HT (quantity * unit_price * (1 - discount))
    pub total_ht: Decimal,
    /// Total TTC (total_ht * (1 + tax_rate))
    pub total_ttc: Decimal,
    /// Whether this is a labor line
    pub is_labor: bool,
    /// Part ID (if this line is for a part)
    pub part_id: Option<Uuid>,
    /// Labor rate ID (if this line is for labor)
    pub labor_rate_id: Option<Uuid>,
}

impl InvoiceLine {
    /// Creates a new invoice line
    pub fn new(
        invoice_id: Uuid,
        line_number: i32,
        description: String,
        quantity: Decimal,
        unit_price: Decimal,
        tax_rate: Decimal,
        is_labor: bool,
    ) -> Self {
        let discount = Decimal::ZERO;
        let quantity_dec = quantity;
        let unit_price_dec = unit_price;
        
        // Calculate using Decimal arithmetic
        let total_ht = quantity_dec * unit_price_dec * (Decimal::ONE - discount);
        let total_ttc = total_ht * (Decimal::ONE + tax_rate);
        
        Self {
            id: Uuid::new_v4(),
            invoice_id,
            line_number,
            description,
            quantity: quantity_dec,
            unit_price: unit_price_dec,
            tax_rate,
            discount: Some(discount),
            total_ht,
            total_ttc,
            is_labor,
            part_id: None,
            labor_rate_id: None,
        }
    }

    /// Recalculates totals
    pub fn recalculate(&mut self) {
        let discount = self.discount.unwrap_or(Decimal::ZERO);
        self.total_ht = self.quantity * self.unit_price * (Decimal::ONE - discount);
        self.total_ttc = self.total_ht * (Decimal::ONE + self.tax_rate);
    }
}

/// Invoice entity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Invoice {
    /// Unique identifier
    pub id: Uuid,
    /// Invoice number (e.g., FACT-2024-0001)
    pub invoice_number: String,
    /// Invoice type
    pub invoice_type: InvoiceType,
    /// Client ID
    pub client_id: Uuid,
    /// Vehicle ID (optional)
    pub vehicle_id: Option<Uuid>,
    /// Invoice date
    pub date: DateTime<Utc>,
    /// Due date
    pub due_date: DateTime<Utc>,
    /// Status
    pub status: InvoiceStatus,
    /// Payment method
    pub payment_method: Option<PaymentMethod>,
    /// Payment date
    pub payment_date: Option<DateTime<Utc>>,
    /// Subtotal (HT)
    pub subtotal: Decimal,
    /// Tax amount
    pub tax_amount: Decimal,
    /// Total (TTC)
    pub total: Decimal,
    /// Discount amount
    pub discount: Option<Decimal>,
    /// Notes
    pub notes: Option<String>,
    /// Timestamps
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    /// Created by user ID
    pub created_by: Uuid,
    /// Whether this invoice has been exported
    pub is_exported: bool,
    /// Export date
    pub exported_at: Option<DateTime<Utc>>,
    /// External reference (from other accounting software)
    pub external_reference: Option<String>,
    // Navigation properties (not stored in DB)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client: Option<Client>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vehicle: Option<Vehicle>,
}

impl Invoice {
    /// Creates a new invoice
    pub fn new(
        invoice_number: String,
        invoice_type: InvoiceType,
        client_id: Uuid,
        date: DateTime<Utc>,
        due_date: DateTime<Utc>,
        created_by: Uuid,
    ) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            invoice_number,
            invoice_type,
            client_id,
            vehicle_id: None,
            date,
            due_date,
            status: InvoiceStatus::Draft,
            payment_method: None,
            payment_date: None,
            subtotal: Decimal::ZERO,
            tax_amount: Decimal::ZERO,
            total: Decimal::ZERO,
            discount: None,
            notes: None,
            created_at: now,
            updated_at: now,
            created_by,
            is_exported: false,
            exported_at: None,
            external_reference: None,
            client: None,
            vehicle: None,
        }
    }

    /// Recalculates totals from lines
    pub fn recalculate_totals(&mut self, lines: &[InvoiceLine]) {
        let subtotal: Decimal = lines.iter().map(|l| l.total_ht).sum();
        let tax_amount: Decimal = lines.iter().map(|l| l.total_ttc - l.total_ht).sum();
        let total: Decimal = lines.iter().map(|l| l.total_ttc).sum();
        
        let discount = self.discount.unwrap_or(Decimal::ZERO);
        
        self.subtotal = subtotal;
        self.tax_amount = tax_amount;
        self.total = total - discount;
    }

    /// Whether the invoice is overdue
    pub fn is_overdue(&self) -> bool {
        self.status != InvoiceStatus::Paid 
            && self.status != InvoiceStatus::Cancelled
            && self.due_date < Utc::now()
    }

    /// Whether the invoice is fully paid
    pub fn is_fully_paid(&self) -> bool {
        self.status == InvoiceStatus::Paid
    }

    /// Remaining amount to pay
    pub fn remaining_amount(&self) -> Decimal {
        if self.is_fully_paid() {
            Decimal::ZERO
        } else {
            self.total
        }
    }
}
