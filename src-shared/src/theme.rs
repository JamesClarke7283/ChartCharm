use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ThemeError {
    #[error("Failed to create table: {0}")]
    CreateTableError(String),
    #[error("Failed to insert theme: {0}")]
    InsertError(String),
    #[error("Failed to retrieve theme: {0}")]
    RetrieveError(String),
    #[error("Failed to connect to database: {0}")]
    ConnectionError(String, String),
    #[error("Failed to decode theme")]
    DecodeError,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Theme {
    pub id: u8,
    pub name: String,
}
