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
    // Get Current workinf directory and go back one level
    let cwd = match std::env::current_dir() {
        Ok(cwd) => cwd,
        Err(e) => {
            eprintln!(
                "Failed to get current working directory while initilizing database: {:?}",
                e
            );
            return Err(Box::new(e));
        }
    };
    let cwd_parent = cwd.parent().unwrap();

    let cwd_str = cwd_parent.to_str().unwrap();
    println!("Current working directory: {:?}", cwd_str);

    let sql_path = format!("../src-db/src/sql/create_tables.sql");
    eprintln!("Reading SQL script from {}", sql_path);
    let sql_script = match fs::read_to_string(sql_path) {
        Ok(script) => script,
        Err(e) => {
            eprintln!("Failed to read SQL script: {:?}", e);
            return Err(Box::new(e));
        }
    };
    query(&sql_script).execute(db).await?;
    Ok(())
}

pub async fn check_empty(db: &mut SqliteConnection) -> Result<bool, Box<dyn std::error::Error>> {
    let sql_path = "../src-db/src/sql/count_theme_types.sql";
    eprintln!("Reading SQL script from {}", sql_path);
    let sql_script = match fs::read_to_string(sql_path) {
        Ok(script) => script,
        Err(e) => {
            eprintln!("Failed to read SQL script: {:?}", e);
            return Err(Box::new(e));
        }
    };
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

    let db_string = format!("sqlite://{}", db_path.to_str().unwrap());

    if !db_path.exists() {
        println!("DB file does not exist, creating it now.");
        match fs::File::create(&db_path) {
            Ok(_) => println!("DB file created successfully"),
            Err(e) => println!("Failed to create DB file: {:?}", e),
        }
    } else {
        println!("DB file exists");
    }

    Ok(db_string)
}
