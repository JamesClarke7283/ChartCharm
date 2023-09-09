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

#[derive(Debug, Error, Serialize, Deserialize)]
pub enum SettingsError {
    #[error("Failed to retrieve setting: {0}")]
    RetrieveError(String),
    #[error("Failed to connect to database {0} for setting: {1}")]
    ConnectionError(String, String),
    #[error("Failed to update setting: {0}")]
    UpdateError(String),
}

#[derive(Debug, Error, Serialize, Deserialize)]
pub enum ChartError {
    #[error("Failed to retrieve chart: {0}")]
    RetrieveError(String),
    #[error("Failed to connect to database {0} for chart: {1}")]
    ConnectionError(String, String),
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Project {
    pub id: u16,
    pub name: String,
    pub description: String,
    pub created_at: chrono::DateTime<Utc>,
    pub updated_at: chrono::DateTime<Utc>,
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

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Theme {
    pub id: u8,
    pub name: String,
}
