use std::time::Duration;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum PiError {
    #[error("HTTP request failed: {0}")]
    Http(#[from] reqwest::Error),

    #[error("JSON serialization failed: {0}")]
    Json(#[from] serde_json::Error),

    #[error("Pi Network API error: {error_name} - {error_message}")]
    PiNetwork {
        error_name: String,
        error_message: String,
        payment: Option<crate::models::PaymentDto>,
    },

    #[error("Authentication failed: {0}")]
    Authentication(String),

    #[error("Invalid configuration: {0}")]
    Configuration(String),

    #[error("Stellar operation failed: {0}")]
    Stellar(String),

    #[error("Insufficient balance: available {available}, required {required}")]
    InsufficientBalance { available: f64, required: f64 },

    #[error("Timeout occurred after {duration:?}")]
    Timeout { duration: Duration },
}

pub type Result<T> = std::result::Result<T, PiError>;