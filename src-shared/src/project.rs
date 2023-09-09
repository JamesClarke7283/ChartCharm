use chrono::Utc;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Error, Serialize, Deserialize)]
pub enum ProjectError {
    #[error("Failed to retrieve project: {0}")]
    RetrieveError(String),
    #[error("Failed to connect to database {0} for project: {1}")]
    ConnectionError(String, String),
    #[error("Failed to insert project: {0}")]
    InsertError(String),
    #[error("Failed to delete project: {0}")]
    DeleteError(String),
    #[error("Failed to update project: {0}")]
    UpdateError(String),
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Project {
    pub id: u16,
    pub name: String,
    pub description: String,
    pub created_at: chrono::DateTime<Utc>,
    pub updated_at: chrono::DateTime<Utc>,
}
