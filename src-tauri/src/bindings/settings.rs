use chartcharm_database::{get_connection, is_db_populated};
use chartcharm_shared::settings::{Setting, SettingsError};
use rusqlite::{params, Result};

pub fn create_settings_table() -> Result<(), SettingsError> {
    let conn = get_connection()
        .map_err(|e| SettingsError::ConnectionError("N/A".to_string(), e.to_string()))?;
    if is_db_populated() {
        return Ok(());
    } else {
        conn.execute(
            "CREATE TABLE IF NOT EXISTS settings (
            id INTEGER PRIMARY KEY,
            name TEXT NOT NULL UNIQUE,
            value TEXT NOT NULL
        );",
            params![],
        )
        .map_err(|e| SettingsError::CreateError(e.to_string()))?;
        Ok(())
    }
}

pub fn populate_settings_table() -> Result<(), SettingsError> {
    let conn = get_connection()
        .map_err(|e| SettingsError::ConnectionError("N/A".to_string(), e.to_string()))?;
    if is_db_populated() {
        return Ok(());
    } else {
        conn.execute(
            "INSERT INTO settings (name, value) VALUES ('theme_selected', '1');",
            params![],
        )
        .map_err(|e| SettingsError::InsertError(e.to_string()))?;

        conn.execute(
            "INSERT INTO settings (name, value) VALUES ('is_db_populated', '0');",
            params![],
        )
        .map_err(|e| SettingsError::InsertError(e.to_string()))?;

        Ok(())
    }
}

// The rest of the methods will look for settings by their name instead of id.
#[tauri::command]
pub fn update_settings_theme(theme_id: &str) -> Result<(), SettingsError> {
    let conn = get_connection()
        .map_err(|e| SettingsError::ConnectionError("N/A".to_string(), e.to_string()))?;

    conn.execute(
        "UPDATE settings SET value = ?1 WHERE name = 'theme_selected';",
        params![theme_id],
    )
    .map_err(|e| SettingsError::UpdateError(e.to_string()))?;

    Ok(())
}

#[tauri::command]
pub fn query_settings_theme() -> Result<String, SettingsError> {
    let conn = get_connection()
        .map_err(|e| SettingsError::ConnectionError("N/A".to_string(), e.to_string()))?;

    let theme_id: String = conn
        .query_row(
            "SELECT value FROM settings WHERE name = 'theme_selected';",
            params![],
            |row| row.get(0),
        )
        .map_err(|e| SettingsError::RetrieveError(e.to_string()))?;

    Ok(theme_id)
}
