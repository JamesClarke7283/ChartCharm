// src-db/src/models.rs

use dirs::config_dir;
use prsqlite::*;
use std::fs;

pub fn populate(db: &mut Connection) -> Result<(), Box<dyn std::error::Error>> {
    println!("Populating database");
    // Your database population logic here...
    Ok(())
}

pub fn initialize(db: &mut Connection) -> Result<(), Box<dyn std::error::Error>> {
    println!("Initializing database");
    let sql_script = fs::read_to_string("./sql/create_tables.sql")?;
    let mut stmt = db.prepare(&sql_script)?;
    stmt.execute()?;
    Ok(())
}

pub fn check_empty_sync(db: &mut Connection) -> Result<bool, String> {
    let sql_script = fs::read_to_string("sql/count_themes.sql").expect("Unable to read SQL file");

    let mut stmt = db.prepare(&sql_script).unwrap();
    let mut rows = stmt.execute().unwrap();

    let row = match rows.next_row() {
        Ok(Some(row)) => row,
        Ok(None) => return Err("No rows returned by the query".to_string()),
        Err(_) => return Err("Error executing SQL query".to_string()),
    };

    let columns = row.parse().unwrap();
    let theme_count_value = columns.get(0).to_owned();

    let theme_count = match theme_count_value {
        Value::Integer(i) => i,
        _ => return Err("Unexpected value type, expected an Integer".to_string()),
    };

    println!("Theme count: {}", theme_count);
    Ok(theme_count == 0)
}

pub fn check_empty(db: &mut Connection) -> Result<bool, Box<dyn std::error::Error>> {
    match check_empty_sync(db) {
        Ok(result) => Ok(result),
        Err(e) => {
            eprintln!("{}", e);
            Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, e)))
        }
    }
}

pub fn db_string() -> String {
    let base_dir = config_dir().expect("Could not find config directory");
    println!("Base dir: {:?}", base_dir);

    let mut db_dir = base_dir.clone();
    db_dir.push("ChartCharm");
    if !db_dir.exists() {
        fs::create_dir_all(&db_dir).expect("Failed to create directory");
    }
    println!("DB dir: {:?}", db_dir);

    let mut db_path = db_dir;
    db_path.push("database.sqlite3");
    let db_string = format!("{}", db_path.to_str().unwrap());
    println!("DB string: {}", db_string);

    if !db_path.exists() {
        fs::File::create(&db_path).expect("Failed to create file");
        println!("DB file created");
    } else {
        println!("DB file exists");
    }

    db_string
}
