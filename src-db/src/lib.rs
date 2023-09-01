use sea_orm::Database;

pub mod models;

pub async fn init_db() -> Result<(), Box<dyn std::error::Error>> {
    // Log the database string for debugging
    let database_url = models::db_string()?;
    eprintln!("Connecting to database at {}", database_url);

    // Connect to the database
    let db = Database::connect(&database_url).await?;
    eprintln!("Successfully connected to database");

    // Create tables if they don't exist
    match models::initialize(&db).await {
        Ok(_) => eprintln!("Successfully initialized database"),
        Err(e) => {
            eprintln!("Failed to initialize the database: {:?}", e);
            return Err(Box::new(e));
        }
    }

    // Populate database with default data
    if models::check_empty(&db).await? {
        match models::populate(&db).await {
            Ok(_) => eprintln!("Successfully populated database"),
            Err(e) => eprintln!("Failed to populate the database: {:?}", e),
        }
    }

    Ok(())
}
