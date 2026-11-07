//! Repository for Invoice entities

use sqlx::{Row, sqlite::SqliteRow};
use uuid::Uuid;

use garage_compta_core::models::{Invoice, InvoiceLine, InvoiceStatus};
use garage_compta_core::CoreError;
use crate::connection::DatabasePool;

/// Invoice repository
#[derive(Debug, Clone)]
pub struct InvoiceRepository {
    pool: DatabasePool,
}

impl InvoiceRepository {
    /// Creates a new invoice repository
    pub fn new(pool: DatabasePool) -> Self {
        Self { pool }
    }

    /// Creates a new invoice
    pub async fn create(&self, invoice: &Invoice) -> Result<Invoice, CoreError> {
        let discount_str = invoice.discount.map(|d| d.to_string());
        
        let invoice = sqlx::query_as::<_, Invoice>(
            r#"
            INSERT INTO invoices (
                id, invoice_number, invoice_type, client_id, vehicle_id,
                date, due_date, status, payment_method, payment_date,
                subtotal, tax_amount, total, discount, notes,
                created_at, updated_at, created_by, is_exported, exported_at, external_reference
            ) VALUES (
                $1, $2, $3, $4, $5,
                $6, $7, $8, $9, $10,
                $11, $12, $13, $14, $15,
                $16, $17, $18, $19, $20, $21
            )
            RETURNING *
            "#,
        )
        .bind(&invoice.id)
        .bind(&invoice.invoice_number)
        .bind(&invoice.invoice_type)
        .bind(&invoice.client_id)
        .bind(&invoice.vehicle_id)
        .bind(&invoice.date)
        .bind(&invoice.due_date)
        .bind(&invoice.status)
        .bind(&invoice.payment_method)
        .bind(&invoice.payment_date)
        .bind(invoice.subtotal.to_string())
        .bind(invoice.tax_amount.to_string())
        .bind(invoice.total.to_string())
        .bind(&discount_str)
        .bind(&invoice.notes)
        .bind(&invoice.created_at)
        .bind(&invoice.updated_at)
        .bind(&invoice.created_by)
        .bind(&invoice.is_exported)
        .bind(&invoice.exported_at)
        .bind(&invoice.external_reference)
        .fetch_one(&self.pool)
        .await?;

        Ok(invoice)
    }

    /// Gets an invoice by ID
    pub async fn get(&self, id: Uuid) -> Result<Option<Invoice>, CoreError> {
        let invoice = sqlx::query_as::<_, Invoice>(
            "SELECT * FROM invoices WHERE id = $1 LIMIT 1"
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(invoice)
    }

    /// Gets all invoices
    pub async fn get_all(&self) -> Result<Vec<Invoice>, CoreError> {
        let invoices = sqlx::query_as::<_, Invoice>(
            "SELECT * FROM invoices ORDER BY date DESC, invoice_number DESC"
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(invoices)
    }

    /// Updates an invoice
    pub async fn update(&self, invoice: &Invoice) -> Result<Invoice, CoreError> {
        let discount_str = invoice.discount.map(|d| d.to_string());
        
        let invoice = sqlx::query_as::<_, Invoice>(
            r#"
            UPDATE invoices SET
                invoice_number = $2, invoice_type = $3, client_id = $4, vehicle_id = $5,
                date = $6, due_date = $7, status = $8, payment_method = $9, payment_date = $10,
                subtotal = $11, tax_amount = $12, total = $13, discount = $14, notes = $15,
                updated_at = $16, is_exported = $17, exported_at = $18, external_reference = $19
            WHERE id = $1
            RETURNING *
            "#,
        )
        .bind(&invoice.id)
        .bind(&invoice.invoice_number)
        .bind(&invoice.invoice_type)
        .bind(&invoice.client_id)
        .bind(&invoice.vehicle_id)
        .bind(&invoice.date)
        .bind(&invoice.due_date)
        .bind(&invoice.status)
        .bind(&invoice.payment_method)
        .bind(&invoice.payment_date)
        .bind(invoice.subtotal.to_string())
        .bind(invoice.tax_amount.to_string())
        .bind(invoice.total.to_string())
        .bind(&discount_str)
        .bind(&invoice.notes)
        .bind(&invoice.updated_at)
        .bind(&invoice.is_exported)
        .bind(&invoice.exported_at)
        .bind(&invoice.external_reference)
        .bind(&invoice.id)
        .fetch_one(&self.pool)
        .await?;

        Ok(invoice)
    }

    /// Deletes an invoice
    pub async fn delete(&self, id: Uuid) -> Result<bool, CoreError> {
        let result = sqlx::query_scalar::<_, i64>(
            "DELETE FROM invoices WHERE id = $1"
        )
        .bind(id)
        .fetch_one(&self.pool)
        .await?;

        Ok(result > 0)
    }

    /// Gets invoice by number
    pub async fn get_by_number(&self, number: &str) -> Result<Option<Invoice>, CoreError> {
        let invoice = sqlx::query_as::<_, Invoice>(
            "SELECT * FROM invoices WHERE invoice_number = $1 LIMIT 1"
        )
        .bind(number)
        .fetch_optional(&self.pool)
        .await?;

        Ok(invoice)
    }

    /// Gets invoices by client
    pub async fn get_by_client(&self, client_id: Uuid) -> Result<Vec<Invoice>, CoreError> {
        let invoices = sqlx::query_as::<_, Invoice>(
            "SELECT * FROM invoices WHERE client_id = $1 ORDER BY date DESC"
        )
        .bind(client_id)
        .fetch_all(&self.pool)
        .await?;

        Ok(invoices)
    }

    /// Gets invoices by status
    pub async fn get_by_status(&self, status: InvoiceStatus) -> Result<Vec<Invoice>, CoreError> {
        let invoices = sqlx::query_as::<_, Invoice>(
            "SELECT * FROM invoices WHERE status = $1 ORDER BY date DESC"
        )
        .bind(&status)
        .fetch_all(&self.pool)
        .await?;

        Ok(invoices)
    }

    /// Gets unpaid invoices
    pub async fn get_unpaid(&self) -> Result<Vec<Invoice>, CoreError> {
        let invoices = sqlx::query_as::<_, Invoice>(
            r#"
            SELECT * FROM invoices 
            WHERE status != 'Paid' AND status != 'Cancelled'
            ORDER BY due_date
            "#
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(invoices)
    }

    /// Gets overdue invoices
    pub async fn get_overdue(&self) -> Result<Vec<Invoice>, CoreError> {
        let invoices = sqlx::query_as::<_, Invoice>(
            r#"
            SELECT * FROM invoices 
            WHERE status != 'Paid' AND status != 'Cancelled' AND due_date < datetime('now')
            ORDER BY due_date
            "#
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(invoices)
    }

    /// Gets the last invoice number
    pub async fn get_last_invoice_number(&self) -> Result<Option<String>, CoreError> {
        let result = sqlx::query(
            "SELECT invoice_number FROM invoices ORDER BY invoice_number DESC LIMIT 1"
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(result.map(|row: SqliteRow| row.get::<Option<String>, _>(0)).flatten())
    }

    /// Checks if an invoice number exists
    pub async fn exists_by_number(&self, number: &str) -> Result<bool, CoreError> {
        let exists: Option<i64> = sqlx::query_scalar(
            "SELECT 1 FROM invoices WHERE invoice_number = $1 LIMIT 1"
        )
        .bind(number)
        .fetch_optional(&self.pool)
        .await?;

        Ok(exists.is_some())
    }

    /// Gets invoices for export (not yet exported)
    pub async fn get_unexported(&self) -> Result<Vec<Invoice>, CoreError> {
        let invoices = sqlx::query_as::<_, Invoice>(
            "SELECT * FROM invoices WHERE is_exported = false ORDER BY date"
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(invoices)
    }

    /// Marks an invoice as exported
    pub async fn mark_as_exported(&self, id: Uuid) -> Result<Invoice, CoreError> {
        let invoice = sqlx::query_as::<_, Invoice>(
            "UPDATE invoices SET is_exported = true, exported_at = datetime('now') WHERE id = $1 RETURNING *"
        )
        .bind(id)
        .fetch_one(&self.pool)
        .await?;

        Ok(invoice)
    }

    // ========================================================================
    // Invoice Lines
    // ========================================================================

    /// Gets all lines for an invoice
    pub async fn get_lines(&self, invoice_id: Uuid) -> Result<Vec<InvoiceLine>, CoreError> {
        let lines = sqlx::query_as::<_, InvoiceLine>(
            "SELECT * FROM invoice_lines WHERE invoice_id = $1 ORDER BY line_number"
        )
        .bind(invoice_id)
        .fetch_all(&self.pool)
        .await?;

        Ok(lines)
    }

    /// Gets a line by ID
    pub async fn get_line(&self, id: Uuid) -> Result<Option<InvoiceLine>, CoreError> {
        let line = sqlx::query_as::<_, InvoiceLine>(
            "SELECT * FROM invoice_lines WHERE id = $1 LIMIT 1"
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(line)
    }

    /// Adds a line to an invoice
    pub async fn add_line(&self, line: &InvoiceLine) -> Result<InvoiceLine, CoreError> {
        let line = sqlx::query_as::<_, InvoiceLine>(
            r#"
            INSERT INTO invoice_lines (
                id, invoice_id, line_number, description, quantity, unit_price,
                tax_rate, discount, total_ht, total_ttc, is_labor, part_id, labor_rate_id
            ) VALUES (
                $1, $2, $3, $4, $5, $6,
                $7, $8, $9, $10, $11, $12, $13
            )
            RETURNING *
            "#,
        )
        .bind(&line.id)
        .bind(&line.invoice_id)
        .bind(&line.line_number)
        .bind(&line.description)
        .bind(line.quantity.to_string())
        .bind(line.unit_price.to_string())
        .bind(line.tax_rate.to_string())
        .bind(line.discount.map(|d| d.to_string()))
        .bind(line.total_ht.to_string())
        .bind(line.total_ttc.to_string())
        .bind(&line.is_labor)
        .bind(&line.part_id)
        .bind(&line.labor_rate_id)
        .fetch_one(&self.pool)
        .await?;

        Ok(line)
    }

    /// Updates a line
    pub async fn update_line(&self, line: &InvoiceLine) -> Result<InvoiceLine, CoreError> {
        let line = sqlx::query_as::<_, InvoiceLine>(
            r#"
            UPDATE invoice_lines SET
                line_number = $2, description = $3, quantity = $4, unit_price = $5,
                tax_rate = $6, discount = $7, total_ht = $8, total_ttc = $9,
                is_labor = $10, part_id = $11, labor_rate_id = $12
            WHERE id = $1
            RETURNING *
            "#,
        )
        .bind(&line.id)
        .bind(&line.line_number)
        .bind(&line.description)
        .bind(line.quantity.to_string())
        .bind(line.unit_price.to_string())
        .bind(line.tax_rate.to_string())
        .bind(line.discount.map(|d| d.to_string()))
        .bind(line.total_ht.to_string())
        .bind(line.total_ttc.to_string())
        .bind(&line.is_labor)
        .bind(&line.part_id)
        .bind(&line.labor_rate_id)
        .bind(&line.id)
        .fetch_one(&self.pool)
        .await?;

        Ok(line)
    }

    /// Deletes a line
    pub async fn delete_line(&self, id: Uuid) -> Result<bool, CoreError> {
        let result = sqlx::query_scalar::<_, i64>(
            "DELETE FROM invoice_lines WHERE id = $1"
        )
        .bind(id)
        .fetch_one(&self.pool)
        .await?;

        Ok(result > 0)
    }

    /// Deletes all lines for an invoice
    pub async fn delete_lines(&self, invoice_id: Uuid) -> Result<i64, CoreError> {
        let result = sqlx::query_scalar::<_, i64>(
            "DELETE FROM invoice_lines WHERE invoice_id = $1"
        )
        .bind(invoice_id)
        .fetch_one(&self.pool)
        .await?;

        Ok(result)
    }

    /// Recalculates invoice totals from its lines
    pub async fn recalculate_totals(&self, invoice_id: Uuid) -> Result<Invoice, CoreError> {
        // Get all lines for the invoice
        let lines = self.get_lines(invoice_id).await?;
        
        // Calculate totals
        let subtotal: rust_decimal::Decimal = lines.iter().map(|l| l.total_ht).sum();
        let tax_amount: rust_decimal::Decimal = lines.iter().map(|l| l.total_ttc - l.total_ht).sum();
        let total: rust_decimal::Decimal = lines.iter().map(|l| l.total_ttc).sum();
        
        // Get the invoice to preserve discount
        let invoice = self.get(invoice_id).await?.ok_or_else(|| {
            CoreError::ValidationError("Invoice not found".to_string())
        })?;
        
        let discount = invoice.discount.unwrap_or(rust_decimal::Decimal::ZERO);
        let final_total = total - discount;
        
        // Update the invoice
        let invoice = sqlx::query_as::<_, Invoice>(
            r#"
            UPDATE invoices SET
                subtotal = $2, tax_amount = $3, total = $4
            WHERE id = $1
            RETURNING *
            "#,
        )
        .bind(invoice_id)
        .bind(subtotal.to_string())
        .bind(tax_amount.to_string())
        .bind(final_total.to_string())
        .fetch_one(&self.pool)
        .await?;

        Ok(invoice)
    }
}
