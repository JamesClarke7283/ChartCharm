use chrono::Utc;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Error, Serialize, Deserialize)]
pub enum ChartError {
    #[error("Failed to retrieve chart: {0}")]
    RetrieveError(String),
    #[error("Failed to connect to database {0} for chart: {1}")]
    ConnectionError(String, String),
    #[error("Failed to insert chart: {0}")]
    InsertError(String),
    #[error("Failed to delete chart: {0}")]
    DeleteError(String),
    #[error("Failed to update chart: {0}")]
    UpdateError(String),
    #[error("Failed to decode chart")]
    DecodeError,
    #[error("Failed to create chart")]
    CreateError,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Chart {
    pub id: u16,
    pub name: String,
    pub description: String,
    pub project: u16,
    pub kind: u8,
    pub created_at: chrono::DateTime<Utc>,
    pub updated_at: chrono::DateTime<Utc>,
}
