pub mod models;
use crate::models::{check_empty, db_string, populate};
use sea_orm::Database;

pub async fn init_db() -> bool {
    // Connect to database
    let db = match Database::connect(&db_string()).await {
        Ok(db) => db,
        Err(_) => {
            return false;
        }
    };

    // Populate database with default data
    if check_empty(&db).await {
        populate(&db).await;
    }

    true
}
