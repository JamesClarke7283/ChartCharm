use sqlx::Connection;
use sqlx::SqliteConnection;
use std::io::{self, ErrorKind};
use std::path::Path;

pub mod models;

pub async fn init_db() -> Result<(), Box<dyn std::error::Error>> {
    // Log the database string for debugging
    let database_url = models::db_string()?;
    eprintln!("Connecting to database at {}", database_url);

    let file_path = Path::new(&database_url);
    eprintln!("Database file path: {:?}", file_path);

    // Connect to the database
    match SqliteConnection::connect("sqlite::memory:").await {
        Ok(connection) => {
            eprintln!("Successfully connected to database");
            let mut db = connection;
            // Create tables if they don't exist
            match models::initialize(&mut db).await {
                Ok(_) => eprintln!("Successfully initialized database"),
                Err(e) => eprintln!("Failed to initialize the database: {:?}", e),
            }

            // Populate database with default data
            if models::check_empty(&mut db).await? {
                match models::populate(&mut db).await {
                    Ok(_) => eprintln!("Successfully populated database"),
                    Err(e) => eprintln!("Failed to populate the database: {:?}", e),
                }
            }
            Ok(())
        }
        Err(e) => {
            eprintln!("Failed to connect to database: {:?}", e);
            Err(Box::new(e))
        }
    }
}
