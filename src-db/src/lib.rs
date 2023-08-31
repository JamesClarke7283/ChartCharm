use sqlx::Connection;
use sqlx::SqliteConnection;
use std::io::{self, ErrorKind};
use std::path::Path;

pub mod models;

pub async fn init_db() -> Result<(), Box<dyn std::error::Error>> {
    // Log the database string for debugging
    let database_url = models::db_string()?;
    println!("Connecting to database at {}", database_url);

    let file_path = Path::new(&database_url);
    println!("Database file path: {:?}", file_path);

    let mut db = SqliteConnection::connect(&database_url).await?;

    // Create tables if they don't exist
    models::initialize(&mut db).await?;

    // Populate database with default data
    if models::check_empty(&mut db).await? {
        models::populate(&mut db).await?;
    }

    Ok(())
}
