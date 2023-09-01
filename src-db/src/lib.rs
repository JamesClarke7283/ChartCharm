use sea_orm::Database;
use sea_orm::DatabaseConnection;
pub mod models;
use chartcharm_database_migration::{Migrator, MigratorTrait};
use dirs::config_dir;
use std::error::Error;
use std::io::{self, ErrorKind};

pub async fn initialize(db: &DatabaseConnection) -> Result<(), sea_orm::error::DbErr> {
    println!("Initializing database");
    // Apply all pending migrations
    Migrator::up(db, None).await?;
    // Initialize your tables
    println!("Successfully created tables");
    Ok(())
}

pub async fn db_string() -> Result<String, io::Error> {
    let base_dir = config_dir()
        .ok_or_else(|| io::Error::new(ErrorKind::NotFound, "Could not find config directory"))?;
    println!("Base dir: {:?}", base_dir);

    let mut db_dir = base_dir.clone();
    db_dir.push("ChartCharm");
    if !db_dir.exists() {
        std::fs::create_dir_all(&db_dir)?;
    }
    println!("DB dir: {:?}", db_dir);

    let mut db_path = db_dir;
    db_path.push("database.sqlite3");

    let db_string = format!("sqlite://{}", db_path.to_str().unwrap());

    if !db_path.exists() {
        println!("DB file does not exist, creating it now.");
        std::fs::File::create(&db_path)?;
        println!("DB file created successfully");
    } else {
        println!("DB file exists");
    }

    Ok(db_string)
}

pub async fn get_connection() -> Result<DatabaseConnection, Box<dyn Error>> {
    let db_string = db_string().await?;
    match Database::connect(&db_string).await {
        Ok(db) => Ok(db),
        Err(e) => {
            eprintln!("Failed to connect to database: {:?}", e);
            Err(Box::new(e))
        }
    }
}

pub async fn init_db() -> Result<(), Box<dyn Error>> {
    let db = get_connection().await?;
    eprintln!("Successfully connected to database");
    initialize(&db).await?;
    eprintln!("Successfully initialized database");
    Ok(())
}
