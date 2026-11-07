//! Error types for Garage Compta

use thiserror::Error;

/// Core error type
#[derive(Error, Debug)]
pub enum CoreError {
    /// Database error
    #[error("Database error: {0}")]
    DatabaseError(String),

    /// Validation error
    #[error("Validation error: {0}")]
    ValidationError(String),

    /// Invoice numbering error
    #[error("Invoice numbering error: {0}")]
    InvoiceNumberingError(String),

    /// Import error
    #[error("Import error: {0}")]
    ImportError(String),

    /// Export error
    #[error("Export error: {0}")]
    ExportError(String),

    /// Calculation error
    #[error("Calculation error: {0}")]
    CalculationError(String),

    /// Conversion error
    #[error("Conversion error: {0}")]
    ConversionError(String),

    /// IO error
    #[error("IO error: {0}")]
    IoError(String),

    /// CSV parsing error
    #[error("CSV parsing error")]
    CsvError(#[from] csv::Error),

    /// XLS parsing error
    #[error("XLS parsing error: {0}")]
    XlsError(String),

    /// Lock error
    #[error("Lock error: {0}")]
    LockError(String),

    /// SQLx error
    #[error("SQLx error: {0}")]
    SqlxError(#[from] sqlx::Error),
}

impl From<std::io::Error> for CoreError {
    fn from(err: std::io::Error) -> Self {
        CoreError::IoError(err.to_string())
    }
}

impl From<rust_decimal::Error> for CoreError {
    fn from(err: rust_decimal::Error) -> Self {
        CoreError::ConversionError(err.to_string())
    }
}

impl From<chrono::ParseError> for CoreError {
    fn from(err: chrono::ParseError) -> Self {
        CoreError::ConversionError(err.to_string())
    }
}
