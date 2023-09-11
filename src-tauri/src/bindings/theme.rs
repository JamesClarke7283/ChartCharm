use chartcharm_database::{get_connection, rusqlite_to_string};
use chartcharm_shared::theme::ThemeError;
use rusqlite::params;

pub fn populate_theme_table() -> Result<(), ThemeError> {
    let mut db = get_connection()
        .map_err(|e| ThemeError::ConnectionError("N/A".to_string(), e.to_string()))?;
    db.execute(
        "INSERT INTO theme (name) VALUES (?1), (?2), (?3)",
        params!["auto", "light", "dark"],
    )
    .map_err(|e| ThemeError::InsertError(e.to_string()))?;
    Ok(())
}

pub fn create_theme_table() -> Result<(), ThemeError> {
    let mut db = get_connection()
        .map_err(|e| ThemeError::ConnectionError("N/A".to_string(), e.to_string()))?;
    db.execute(
        "CREATE TABLE IF NOT EXISTS theme (id INTEGER PRIMARY KEY, name TEXT NOT NULL)",
        [],
    )
    .map_err(|e| ThemeError::CreateTableError(e.to_string()))?;
    Ok(())
}

#[tauri::command]
pub fn insert_theme(name: &str) -> Result<(), ThemeError> {
    let mut db = get_connection()
        .map_err(|e| ThemeError::ConnectionError("N/A".to_string(), e.to_string()))?;
    db.execute("INSERT INTO theme (name) VALUES (?1)", params![name])
        .map_err(|e| ThemeError::InsertError(e.to_string()))?;
    Ok(())
}

#[tauri::command]
pub fn query_theme(id: u8) -> Result<String, ThemeError> {
    let mut db =
        get_connection().map_err(|e| ThemeError::ConnectionError(id.to_string(), e.to_string()))?;
    let mut stmt = db
        .prepare("SELECT name FROM theme WHERE id = ?1")
        .map_err(|e| ThemeError::RetrieveError(e.to_string()))?;

    let mut rows = stmt
        .query(params![id])
        .map_err(|e| ThemeError::RetrieveError(e.to_string()))?;

    let row_result = rows.next();
    match row_result {
        Ok(Some(row)) => {
            let theme_name: String = row
                .get(0)
                .map_err(|e| ThemeError::RetrieveError(e.to_string()))?;
            return Ok(theme_name);
        }
        Ok(None) => Err(ThemeError::RetrieveError("No result found".to_string())),
        Err(e) => Err(ThemeError::RetrieveError(rusqlite_to_string(e))),
    }
}
