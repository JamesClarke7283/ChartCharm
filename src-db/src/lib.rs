use dirs::config_dir;
use rusqlite::Connection;
use std::io::{self, ErrorKind};
use std::path::Path;

pub fn rusqlite_to_string(err: rusqlite::Error) -> String {
    format!("{}", err)
}

fn string_to_anyhow(err: String) -> anyhow::Error {
    anyhow::anyhow!(err)
}

/// # Panics
/// Panics if the either the file creation failes or the `config_dir()` function fails.
/// # Errors
/// Returns an `io::Error` if the either the file creation failes or the `config_dir()` function fails.
pub fn db_string() -> Result<String, io::Error> {
    let mut db_dir = config_dir()
        .ok_or_else(|| io::Error::new(ErrorKind::NotFound, "Could not find config directory"))?;
    println!("Base dir: {db_dir:?}");

    db_dir.push("ChartCharm");
    if !db_dir.exists() {
        std::fs::create_dir_all(&db_dir)?;
    }
    println!("DB dir: {db_dir:?}");

    let mut db_path = db_dir;
    db_path.push("database.sqlite3");

    let db_string = db_path.to_str().unwrap();

    if db_path.exists() {
        println!("DB file exists");
    } else {
        println!("DB file does not exist, creating it now.");
        std::fs::File::create(&db_path)?;
        println!("DB file created successfully");
    }

    Ok(db_string.to_string())
}

/// # Errors
/// Database errors are returned as a `Error` enum
pub fn get_connection() -> Result<Connection, anyhow::Error> {
    let db_string = db_string()?;
    println!("DB string: {db_string:?}");

    match Connection::open(Path::new(&db_string)) {
        Ok(db) => Ok(db),
        Err(e) => {
            eprintln!("Failed to connect to database: {e:?}");
            Err(string_to_anyhow(rusqlite_to_string(e)))
        }
    }
}
