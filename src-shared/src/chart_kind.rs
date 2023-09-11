use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Error, Serialize, Deserialize)]
pub enum ChartKindError {
    #[error("Failed to retrieve chart_kind: {0}")]
    RetrieveError(String),
    #[error("Failed to connect to database {0} for chart_kind: {1}")]
    ConnectionError(String, String),
    #[error("Failed to insert chart_kind: {0}")]
    InsertError(String),
    #[error("Failed to delete chart_kind: {0}")]
    DeleteError(String),
    #[error("Failed to update chart_kind: {0}")]
    UpdateError(String),
    #[error("Failed to decode chart_kind")]
    DecodeError,
    #[error("Failed to create chart_kind: {0}")]
    CreateError(String),
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChartKind {
    pub id: u16,
    pub name: String,
}
