use chartcharm_database::get_connection;
use chartcharm_shared::settings::SettingsError; // Import ThemeError if it's in a different module

pub fn populate_settings_table() -> Result<(), SettingsError> {
    let mut db = get_connection()
        .map_err(|e| SettingsError::ConnectionError("N/A".to_string(), e.to_string()))?;
    let mut stmt = db
        .prepare("INSERT INTO settings (theme_selected) VALUES (1);")
        .map_err(|e| SettingsError::InsertError(e.to_string()))?;

    stmt.execute()
        .map_err(|e| SettingsError::InsertError(e.to_string()))?;
    Ok(())
}

pub fn create_settings_table() -> Result<(), SettingsError> {
    let mut db = get_connection()
        .map_err(|e| SettingsError::ConnectionError("N/A".to_string(), e.to_string()))?;
    let create_table_sql = "CREATE TABLE settings IF NOT EXISTS(
            id INTEGER PRIMARY KEY,
            theme_selected INTEGER NOT NULL,
            FOREIGN KEY (theme_selected) REFERENCES theme (id)
        );";
    let mut stmt = db
        .prepare(create_table_sql)
        .map_err(|e| SettingsError::CreateError(e.to_string()))?;

    stmt.execute()
        .map_err(|e| SettingsError::CreateError(e.to_string()))?;
    Ok(())
}

#[tauri::command]
pub fn update_settings_theme(theme: &str) -> Result<(), SettingsError> {
    let mut db = get_connection()
        .map_err(|e| SettingsError::ConnectionError(e.to_string(), theme.to_string()))?;

    let mut stmt = db
        .prepare(&format!("SELECT id FROM theme WHERE name = '{}';", theme))
        .map_err(|e| SettingsError::RetrieveError(e.to_string()))?;

    let mut rows = stmt
        .execute()
        .map_err(|e| SettingsError::RetrieveError(e.to_string()))?;

    let row = rows.next_row().unwrap().unwrap();
    let columns = row.parse().unwrap();
    let theme_id = columns.get(0).as_string().unwrap(); // Assuming you have an as_string() method

    let mut db2 = get_connection()
        .map_err(|e| SettingsError::ConnectionError(e.to_string(), theme.to_string()))?;

    let mut stmt = db2
        .prepare(&format!(
            "UPDATE settings SET theme_selected = {} WHERE id = 1;",
            theme_id
        ))
        .map_err(|e| SettingsError::UpdateError(e.to_string()))?;

    stmt.execute()
        .map_err(|e| SettingsError::UpdateError(e.to_string()))?;

    Ok(())
}

#[tauri::command]
pub fn query_settings_theme() -> Result<String, SettingsError> {
    let mut db = get_connection()
        .map_err(|e| SettingsError::ConnectionError(e.to_string(), String::new()))?;

    let mut stmt = db
        .prepare("SELECT theme_selected FROM settings WHERE id = 1;")
        .map_err(|e| SettingsError::RetrieveError(e.to_string()))?;

    let mut rows = stmt
        .execute()
        .map_err(|e| SettingsError::RetrieveError(e.to_string()))?;

    let row = rows.next_row().unwrap().unwrap();
    let columns = row.parse().unwrap();
    let theme_id = columns.get(0).as_string().unwrap();

    let mut db2 = get_connection()
        .map_err(|e| SettingsError::ConnectionError(e.to_string(), String::new()))?;

    let mut stmt = db2
        .prepare(&format!("SELECT name FROM theme WHERE id = {};", theme_id))
        .map_err(|e| SettingsError::RetrieveError(e.to_string()))?;

    let mut rows = stmt
        .execute()
        .map_err(|e| SettingsError::RetrieveError(e.to_string()))?;

    let row = rows.next_row().unwrap().unwrap();
    let columns = row.parse().unwrap();
    let theme_name = columns.get(0).as_string().unwrap();

    Ok(theme_name)
}
