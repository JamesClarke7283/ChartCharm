use chrono::Utc;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Error, Serialize, Deserialize)]
pub enum DataPointError {
    #[error("Failed to retrieve datapoint: {0}")]
    RetrieveError(String),
    #[error("Failed to connect to database {0} for datapoint: {1}")]
    ConnectionError(String, String),
    #[error("Failed to insert datapoint: {0}")]
    InsertError(String),
    #[error("Failed to delete datapoint: {0}")]
    DeleteError(String),
    #[error("Failed to update datapoint: {0}")]
    UpdateError(String),
    #[error("Failed to decode datapoint")]
    DecodeError,
    #[error("Failed to create datapoint: {0}")]
    CreateError(String),
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DataPoint {
    pub id: u64,
    pub project: u16,
    pub data: f32,
    pub created_at: chrono::DateTime<Utc>,
    pub updated_at: chrono::DateTime<Utc>,
}
