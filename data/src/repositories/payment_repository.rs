//! Repository for Payment entities

use uuid::Uuid;
use std::str::FromStr;

use garage_compta_core::models::{Payment, PaymentMethod};
use garage_compta_core::CoreError;
use crate::connection::DatabasePool;

/// Payment repository
#[derive(Debug, Clone)]
pub struct PaymentRepository {
    pool: DatabasePool,
}

impl PaymentRepository {
    /// Creates a new payment repository
    pub fn new(pool: DatabasePool) -> Self {
        Self { pool }
    }

    /// Creates a new payment
    pub async fn create(&self, payment: &Payment) -> Result<Payment, CoreError> {
        let payment = sqlx::query_as::<_, Payment>(
            r#"
            INSERT INTO payments (
                id, invoice_id, amount, method, date,
                reference, notes, created_at, updated_at, status
            ) VALUES (
                $1, $2, $3, $4, $5,
                $6, $7, $8, $9, $10
            )
            RETURNING *
            "#,
        )
        .bind(&payment.id)
        .bind(&payment.invoice_id)
        .bind(payment.amount.to_string())
        .bind(&payment.method)
        .bind(&payment.date)
        .bind(&payment.reference)
        .bind(&payment.notes)
        .bind(&payment.created_at)
        .bind(&payment.updated_at)
        .bind(&payment.status)
        .fetch_one(&self.pool)
        .await?;

        Ok(payment)
    }

    /// Gets a payment by ID
    pub async fn get(&self, id: Uuid) -> Result<Option<Payment>, CoreError> {
        let payment = sqlx::query_as::<_, Payment>(
            "SELECT * FROM payments WHERE id = $1 LIMIT 1"
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(payment)
    }

    /// Gets all payments
    pub async fn get_all(&self) -> Result<Vec<Payment>, CoreError> {
        let payments = sqlx::query_as::<_, Payment>(
            "SELECT * FROM payments ORDER BY date DESC"
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(payments)
    }

    /// Updates a payment
    pub async fn update(&self, payment: &Payment) -> Result<Payment, CoreError> {
        let payment = sqlx::query_as::<_, Payment>(
            r#"
            UPDATE payments SET
                invoice_id = $2, amount = $3, method = $4, date = $5,
                reference = $6, notes = $7, updated_at = $8, status = $9
            WHERE id = $1
            RETURNING *
            "#,
        )
        .bind(&payment.id)
        .bind(&payment.invoice_id)
        .bind(payment.amount.to_string())
        .bind(&payment.method)
        .bind(&payment.date)
        .bind(&payment.reference)
        .bind(&payment.notes)
        .bind(&payment.updated_at)
        .bind(&payment.status)
        .bind(&payment.id)
        .fetch_one(&self.pool)
        .await?;

        Ok(payment)
    }

    /// Deletes a payment
    pub async fn delete(&self, id: Uuid) -> Result<bool, CoreError> {
        let result = sqlx::query_scalar::<_, i64>(
            "DELETE FROM payments WHERE id = $1"
        )
        .bind(id)
        .fetch_one(&self.pool)
        .await?;

        Ok(result > 0)
    }

    /// Gets payments by invoice
    pub async fn get_by_invoice(&self, invoice_id: Uuid) -> Result<Vec<Payment>, CoreError> {
        let payments = sqlx::query_as::<_, Payment>(
            "SELECT * FROM payments WHERE invoice_id = $1 ORDER BY date"
        )
        .bind(invoice_id)
        .fetch_all(&self.pool)
        .await?;

        Ok(payments)
    }

    /// Gets payments by date range
    pub async fn get_by_date_range(
        &self,
        start_date: chrono::NaiveDate,
        end_date: chrono::NaiveDate,
    ) -> Result<Vec<Payment>, CoreError> {
        let payments = sqlx::query_as::<_, Payment>(
            r#"
            SELECT * FROM payments 
            WHERE date BETWEEN $1 AND $2
            ORDER BY date
            "#
        )
        .bind(start_date.to_string())
        .bind(end_date.to_string())
        .fetch_all(&self.pool)
        .await?;

        Ok(payments)
    }

    /// Gets payments by method
    pub async fn get_by_method(&self, method: PaymentMethod) -> Result<Vec<Payment>, CoreError> {
        let payments = sqlx::query_as::<_, Payment>(
            "SELECT * FROM payments WHERE method = $1 ORDER BY date DESC"
        )
        .bind(&method)
        .fetch_all(&self.pool)
        .await?;

        Ok(payments)
    }

    /// Gets total payments for an invoice
    pub async fn get_total_for_invoice(&self, invoice_id: Uuid) -> Result<rust_decimal::Decimal, CoreError> {
        let total_str: Option<String> = sqlx::query_scalar(
            "SELECT SUM(amount) FROM payments WHERE invoice_id = $1"
        )
        .bind(invoice_id)
        .fetch_optional(&self.pool)
        .await?;
        
        let total = total_str
            .map(|s| rust_decimal::Decimal::from_str(&s).unwrap_or(rust_decimal::Decimal::ZERO))
            .unwrap_or(rust_decimal::Decimal::ZERO);
        
        Ok(total)
    }

    /// Marks a payment as completed
    pub async fn mark_completed(&self, id: Uuid) -> Result<Payment, CoreError> {
        let payment = sqlx::query_as::<_, Payment>(
            "UPDATE payments SET status = 'Completed', updated_at = datetime('now') WHERE id = $1 RETURNING *"
        )
        .bind(id)
        .fetch_one(&self.pool)
        .await?;

        Ok(payment)
    }
}
