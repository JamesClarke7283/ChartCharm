pub mod models;
use crate::models::{check_empty, db_string, initialize, populate};
use prsqlite::Connection;
use std::path::Path;

pub fn init_db() -> Result<(), Box<dyn std::error::Error>> {
    // Log the database string for debugging
    let database_url = db_string();
    eprintln!("Connecting to database at {}", &database_url);

    let file_path = Path::new(&database_url);
    eprintln!("Database file path: {:?}", file_path);

    // Connect to the database
    let mut db = match Connection::open(file_path) {
        Ok(db) => db,
        Err(e) => {
            eprintln!("Error opening database: {}", e);
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                e.to_string(),
            )) as Box<dyn std::error::Error>);
        }
    };
    // Create tables if they don't exist
    initialize(&mut db)?;

    // Populate database with default data
    if check_empty(&mut db)? {
        // populate(&mut db).await?;
    }

    Ok(())
}
