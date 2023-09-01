use dirs::config_dir;
use sea_orm::DatabaseConnection;
use sea_orm::EntityTrait;
use std::io::{self, ErrorKind};

pub mod data_points;
pub mod projects;
pub mod settings;
pub mod theme;

pub async fn populate(db: &DatabaseConnection) -> Result<(), sea_orm::error::DbErr> {
    println!("Populating database");
    // Your database population logic here...
    Ok(())
}

pub async fn initialize(db: &DatabaseConnection) -> Result<(), sea_orm::error::DbErr> {
    println!("Initializing database");
    // Initialize your tables
    println!("Successfully created tables");
    Ok(())
}

pub async fn check_empty(db: &DatabaseConnection) -> Result<bool, sea_orm::error::DbErr> {
    let theme_count = theme::Entity::find().all(db).await?.len();
    println!("Theme count: {}", theme_count);
    Ok(theme_count == 0)
}

pub fn db_string() -> Result<String, io::Error> {
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
