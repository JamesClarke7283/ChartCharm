use std::error;

use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Error, Serialize, Deserialize)]
pub enum SettingsError {
    #[error("Failed to retrieve setting: {0}")]
    RetrieveError(String),
    #[error("Failed to connect to database {0} for setting: {1}")]
    ConnectionError(String, String),
    #[error("Failed to update setting: {0}")]
    UpdateError(String),
    #[error("Failed to decode setting")]
    DecodeError,
    #[error("Failed to create setting: {0}")]
    CreateError(String),
    #[error("Failed to delete setting: {0}")]
    DeleteError(String),
    #[error("Failed to insert setting: {0}")]
    InsertError(String),
}
