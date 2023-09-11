use chartcharm_shared::settings::SettingsError;
use dirs::config_dir;
use log::{debug, error, trace};
use rusqlite::{params, Connection};
use std::io::{self, ErrorKind};
use std::path::Path;

pub fn does_setting_exist(conn: &Connection, setting_name: &str) -> bool {
    let mut stmt = match conn.prepare("SELECT name FROM settings;") {
        Ok(stmt) => stmt,
        Err(e) => {
            error!("Failed to prepare statement for settings: {:?}", e);
            return false;
        }
    };

    let mut rows = match stmt.query(params![]) {
        Ok(rows) => rows,
        Err(e) => {
            error!("Failed to query for settings: {:?}", e);
            return false;
        }
    };

    while let Ok(Some(row)) = rows.next() {
        let name: String = match row.get(0) {
            Ok(name) => name,
            Err(e) => {
                error!("Failed to get name from row: {:?}", e);
                return false;
            }
        };
        if name == setting_name {
            return true;
        }
    }
    false
}

pub fn does_settings_table_exist(conn: &Connection) -> bool {
    let mut stmt = match conn
        .prepare("SELECT name FROM sqlite_master WHERE type='table' AND name='settings';")
    {
        Ok(stmt) => stmt,
        Err(e) => {
            error!("Failed to prepare statement for settings table: {e:?}");
            return false;
        }
    };
    let mut rows = match stmt.query(params![]) {
        Ok(rows) => rows,
        Err(e) => {
            error!("Failed to query for settings table: {e:?}");
            return false;
        }
    };

    match rows.next() {
        Ok(Some(_)) => true,
        Ok(None) => false,
        Err(e) => {
            error!("Failed to get next row for settings table: {e:?}");
            false
        }
    }
}

pub fn set_is_db_populated(value: bool) -> Result<(), SettingsError> {
    let (db_string, _) = match db_string() {
        Ok((db_string, does_exist)) => (db_string, does_exist),
        Err(e) => {
            error!("Failed to get db string: {e:?}");
            return Err(SettingsError::ConnectionError(
                "N/A".to_string(),
                e.to_string(),
            ));
        }
    };
    let conn = match Connection::open(Path::new(&db_string)) {
        Ok(db) => Ok(db),
        Err(e) => {
            error!("Failed to connect to database: {e:?}");
            Err(string_to_anyhow(rusqlite_to_string(e)))
        }
    }
    .map_err(|e| SettingsError::ConnectionError("N/A".to_string(), e.to_string()))?;

    conn.execute(
        "UPDATE settings SET value = ?1 WHERE name = 'is_db_populated';",
        params![value.to_string()],
    )
    .map_err(|e| SettingsError::UpdateError(e.to_string()))?;

    Ok(())
}

pub fn is_db_populated() -> bool {
    let mut value_bool: bool = false;
    let (db_string, does_exist) = match db_string() {
        Ok((db_string, does_exist)) => (db_string, does_exist),
        Err(e) => {
            error!("Failed to get db string: {:?}", e);
            return false;
        }
    };
    if does_exist {
        let conn = Connection::open(Path::new(&db_string)).unwrap();
        if does_settings_table_exist(&conn) {
            if does_setting_exist(&conn, "is_db_populated") {
                let value_result: Result<String, rusqlite::Error> = conn.query_row(
                    "SELECT value FROM settings WHERE name = 'is_db_populated';",
                    params![],
                    |row| row.get(0),
                );
                if let Ok(value) = value_result {
                    trace!("value_bool: {:?}", value);
                    if let Ok(parsed_value) = value.parse::<bool>() {
                        value_bool = parsed_value;
                    }
                }
            } else {
                error!("is_db_populated setting does not exist");
                value_bool = false;
            }
        } else {
            error!("settings table does not exist");
            value_bool = false;
        }
    } else {
        error!("DB file does not exist");
        value_bool = false;
    }
    trace!("is_db_populated: {:?}", value_bool);
    value_bool
}

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
pub fn db_string() -> Result<(String, bool), io::Error> {
    let mut does_exist = false;
    let mut db_dir = config_dir()
        .ok_or_else(|| io::Error::new(ErrorKind::NotFound, "Could not find config directory"))?;
    trace!("Base dir: {db_dir:?}");

    db_dir.push("ChartCharm");
    if !db_dir.exists() {
        std::fs::create_dir_all(&db_dir)?;
    }
    trace!("DB dir: {db_dir:?}");

    let mut db_path = db_dir;
    db_path.push("database.sqlite3");

    let db_string = db_path.to_str().unwrap();

    if db_path.exists() {
        debug!("DB file exists");
        does_exist = true;
    } else {
        debug!("DB file does not exist, creating it now.");
        std::fs::File::create(&db_path)?;
        debug!("DB file created successfully");
    }

    Ok((db_string.to_string(), does_exist))
}

pub fn get_connection() -> Result<Connection, anyhow::Error> {
    let (db_string, _) = db_string()?;

    match Connection::open(Path::new(&db_string)) {
        Ok(conn) => Ok(conn),
        Err(e) => {
            error!("Failed to connect to database: {e:?}");
            Err(string_to_anyhow(rusqlite_to_string(e)))
        }
    }
}
