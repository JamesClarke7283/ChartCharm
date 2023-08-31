use dirs::config_dir;
use sqlx::{query, Row, SqliteConnection};
use std::fs;
use std::fs::File;
use std::io::{self, ErrorKind};

pub async fn populate(db: &mut SqliteConnection) -> Result<(), Box<dyn std::error::Error>> {
    println!("Populating database");
    // Your database population logic here...
    Ok(())
}

pub async fn initialize(db: &mut SqliteConnection) -> Result<(), Box<dyn std::error::Error>> {
    println!("Initializing database");
    let sql_script = fs::read_to_string("src-db/src/sql/create_tables.sql")?;
    query(&sql_script).execute(db).await?;
    Ok(())
}

pub async fn check_empty(db: &mut SqliteConnection) -> Result<bool, Box<dyn std::error::Error>> {
    let sql_script = fs::read_to_string("src-db/src/sql/count_theme_types.sql")?;
    let row = query(&sql_script).fetch_one(db).await?;
    let theme_count: i64 = row.try_get(0)?;
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
        fs::create_dir_all(&db_dir)?;
    }
    println!("DB dir: {:?}", db_dir);

    let mut db_path = db_dir;
    db_path.push("database.sqlite3");

    match File::open(&db_path) {
        Ok(_) => println!("Successfully opened the file"),
        Err(e) => println!("Failed to open the file: {:?}", e),
    }

    let db_string = format!("sqlite:/{}", db_path.to_str().unwrap());

    if !db_path.exists() {
        println!("DB file created");
        fs::File::create(&db_path)?;
    } else {
        println!("DB file exists");
    }

    Ok(db_string)
}
