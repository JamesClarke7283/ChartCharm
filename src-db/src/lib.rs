pub mod models;
use crate::models::{check_empty, db_string, populate};
use prsqlite::Connection;
use std::path::Path;

pub async fn init_db() {
    // Log the database string for debugging
    let database_url = db_string().await;
    eprintln!("Connecting to database at {}", database_url);

    let file_path = Path::new(&database_url);

    // Connect to the database
    let mut db = match Connection::open(file_path) {
        Ok(db) => db,
        Err(e) => {
            panic!("Error connecting to database: {}", e);
        }
    };
    // Create tables if they don't exist
    match db.execute_batch(include_str!("../sql/create_tables.sql")) {
        Ok(_) => (),
        Err(e) => {
            panic!("Error creating tables: {}", e);
        }
    };

    // Populate database with default data
    if check_empty(&mut db).await {
        populate(&mut db).await;
    }
}
