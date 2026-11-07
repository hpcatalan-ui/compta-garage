//! Invoice routes

use axum::{Router, routing::{get, post, put, delete}};
use axum::extract::{State, Path};
use axum::Json;
use uuid::Uuid;
use garage_compta_core::models::{Invoice, InvoiceStatus};
use garage_compta_data::repositories::InvoiceRepository;
use crate::state::AppState;
use crate::middleware::ApiError;

/// Creates the invoice router
pub fn router(state: AppState) -> Router<AppState> {
    Router::new().with_state(state)
        .route("/", get(get_invoices))
        .route("/", post(create_invoice))
        .route("/:id", get(get_invoice))
        .route("/:id", put(update_invoice))
        .route("/:id", delete(delete_invoice))
        .route("/client/:client_id", get(get_invoices_by_client))
        .route("/status/:status", get(get_invoices_by_status))
        .route("/number/:number", get(get_invoice_by_number))
        .route("/unpaid", get(get_unpaid_invoices))
        .route("/overdue", get(get_overdue_invoices))
}

/// Gets all invoices
async fn get_invoices(State(state): State<AppState>) -> Result<Json<Vec<Invoice>>, ApiError> {
    let repo = InvoiceRepository::new(state.db.pool.clone());
    let invoices = repo.get_all().await?;
    Ok(Json(invoices))
}

/// Gets an invoice by ID
async fn get_invoice(Path(id): Path<Uuid>, State(state): State<AppState>) -> Result<Json<Invoice>, ApiError> {
    let repo = InvoiceRepository::new(state.db.pool.clone());
    let invoice = repo.get(id).await?.ok_or_else(|| {
        ApiError::new(axum::http::StatusCode::NOT_FOUND, "Invoice not found")
    })?;
    Ok(Json(invoice))
}

/// Creates a new invoice
async fn create_invoice(State(state): State<AppState>, Json(invoice): Json<Invoice>) -> Result<Json<Invoice>, ApiError> {
    let repo = InvoiceRepository::new(state.db.pool.clone());
    let invoice = repo.create(&invoice).await?;
    Ok(Json(invoice))
}

/// Updates an invoice
async fn update_invoice(Path(id): Path<Uuid>, State(state): State<AppState>, Json(invoice): Json<Invoice>) -> Result<Json<Invoice>, ApiError> {
    let repo = InvoiceRepository::new(state.db.pool.clone());
    let mut invoice = invoice;
    invoice.id = id;
    invoice.updated_at = chrono::Utc::now();
    let invoice = repo.update(&invoice).await?;
    Ok(Json(invoice))
}

/// Deletes an invoice
async fn delete_invoice(Path(id): Path<Uuid>, State(state): State<AppState>) -> Result<Json<bool>, ApiError> {
    let repo = InvoiceRepository::new(state.db.pool.clone());
    let deleted = repo.delete(id).await?;
    Ok(Json(deleted))
}

/// Gets invoices by client
async fn get_invoices_by_client(Path(client_id): Path<Uuid>, State(state): State<AppState>) -> Result<Json<Vec<Invoice>>, ApiError> {
    let repo = InvoiceRepository::new(state.db.pool.clone());
    let invoices = repo.get_by_client(client_id).await?;
    Ok(Json(invoices))
}

/// Gets invoices by status
async fn get_invoices_by_status(Path(status): Path<String>, State(state): State<AppState>) -> Result<Json<Vec<Invoice>>, ApiError> {
    let repo = InvoiceRepository::new(state.db.pool.clone());
    let status_enum: InvoiceStatus = status.parse().map_err(|_| {
        ApiError::new(axum::http::StatusCode::BAD_REQUEST, "Invalid status")
    })?;
    let invoices = repo.get_by_status(status_enum).await?;
    Ok(Json(invoices))
}

/// Gets an invoice by number
async fn get_invoice_by_number(Path(number): Path<String>, State(state): State<AppState>) -> Result<Json<Invoice>, ApiError> {
    let repo = InvoiceRepository::new(state.db.pool.clone());
    let invoice = repo.get_by_number(&number).await?.ok_or_else(|| {
        ApiError::new(axum::http::StatusCode::NOT_FOUND, "Invoice not found")
    })?;
    Ok(Json(invoice))
}

/// Gets unpaid invoices
async fn get_unpaid_invoices(State(state): State<AppState>) -> Result<Json<Vec<Invoice>>, ApiError> {
    let repo = InvoiceRepository::new(state.db.pool.clone());
    let invoices = repo.get_unpaid().await?;
    Ok(Json(invoices))
}

/// Gets overdue invoices
async fn get_overdue_invoices(State(state): State<AppState>) -> Result<Json<Vec<Invoice>>, ApiError> {
    let repo = InvoiceRepository::new(state.db.pool.clone());
    let invoices = repo.get_overdue().await?;
    Ok(Json(invoices))
}
