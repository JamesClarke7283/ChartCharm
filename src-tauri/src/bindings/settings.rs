use chartcharm_database::get_connection;
use chartcharm_shared::settings::SettingsError;
use rusqlite::{params, Result};

pub fn populate_settings_table() -> Result<(), SettingsError> {
    let conn = match get_connection() {
        Ok(conn) => conn,
        Err(e) => {
            return Err(SettingsError::ConnectionError(
                "N/A".to_string(),
                e.to_string(),
            ))
        }
    };

    conn.execute(
        "INSERT INTO settings (theme_selected) VALUES (1);",
        params![],
    )
    .map_err(|e| SettingsError::InsertError(e.to_string()))?;
    Ok(())
}

pub fn create_settings_table() -> Result<(), SettingsError> {
    let conn = match get_connection() {
        Ok(conn) => conn,
        Err(e) => {
            return Err(SettingsError::ConnectionError(
                "N/A".to_string(),
                e.to_string(),
            ))
        }
    };
    conn.execute(
        "CREATE TABLE IF NOT EXISTS settings (
            id INTEGER PRIMARY KEY,
            theme_selected INTEGER NOT NULL,
            FOREIGN KEY (theme_selected) REFERENCES theme (id)
        );",
        params![],
    )
    .map_err(|e| SettingsError::CreateError(e.to_string()))?;
    Ok(())
}

#[tauri::command]
pub fn update_settings_theme(theme: &str) -> Result<(), SettingsError> {
    let conn = match get_connection() {
        Ok(conn) => conn,
        Err(e) => {
            return Err(SettingsError::ConnectionError(
                "N/A".to_string(),
                e.to_string(),
            ))
        }
    };
    let theme_id: i32 = conn
        .query_row(
            "SELECT id FROM theme WHERE name = ?1",
            params![theme],
            |row| row.get(0),
        )
        .map_err(|e| SettingsError::RetrieveError(e.to_string()))?;

    conn.execute(
        "UPDATE settings SET theme_selected = ?1 WHERE id = 1;",
        params![theme_id],
    )
    .map_err(|e| SettingsError::UpdateError(e.to_string()))?;
    Ok(())
}

#[tauri::command]
pub fn query_settings_theme() -> Result<String, SettingsError> {
    let conn = match get_connection() {
        Ok(conn) => conn,
        Err(e) => {
            return Err(SettingsError::ConnectionError(
                "N/A".to_string(),
                e.to_string(),
            ))
        }
    };
    let theme_id: i32 = conn
        .query_row(
            "SELECT theme_selected FROM settings WHERE id = 1",
            params![],
            |row| row.get(0),
        )
        .map_err(|e| SettingsError::RetrieveError(e.to_string()))?;

    let theme_name: String = conn
        .query_row(
            "SELECT name FROM theme WHERE id = ?1",
            params![theme_id],
            |row| row.get(0),
        )
        .map_err(|e| SettingsError::RetrieveError(e.to_string()))?;

    Ok(theme_name)
}
