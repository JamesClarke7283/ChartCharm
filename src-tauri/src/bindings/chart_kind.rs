use chartcharm_database::{get_connection, is_db_populated};
use chartcharm_shared::chart_kind::ChartKindError;
use log::debug;
use rusqlite::{params, Result};

pub fn populate_chartkind_table() -> Result<(), ChartKindError> {
    let conn = match get_connection() {
        Ok(conn) => conn,
        Err(e) => {
            return Err(ChartKindError::ConnectionError(
                "N/A".to_string(),
                e.to_string(),
            ))
        }
    };
    if is_db_populated() {
        return Ok(());
    } else {
        debug!("Populating chart_kind table");
        conn.execute(
            "INSERT INTO chart_kind (name) VALUES (?1);",
            params!["line"],
        )
        .map_err(|e| ChartKindError::InsertError(e.to_string()))?;

        Ok(())
    }
}

pub fn create_chartkind_table() -> Result<(), ChartKindError> {
    let conn = match get_connection() {
        Ok(conn) => conn,
        Err(e) => {
            return Err(ChartKindError::ConnectionError(
                "N/A".to_string(),
                e.to_string(),
            ))
        }
    };
    if is_db_populated() {
        return Ok(());
    } else {
        debug!("Creating chart_kind table");

        conn.execute(
            "CREATE TABLE IF NOT EXISTS chart_kind (
            id INTEGER PRIMARY KEY,
            name TEXT NOT NULL UNIQUE
        );",
            params![],
        )
        .map_err(|e| ChartKindError::CreateError(e.to_string()))?;

        Ok(())
    }
}

#[tauri::command]
pub fn query_chart_kind(id: u8) -> Result<String, ChartKindError> {
    let conn = match get_connection() {
        Ok(conn) => conn,
        Err(e) => {
            return Err(ChartKindError::ConnectionError(
                e.to_string(),
                id.to_string(),
            ))
        }
    };

    let name: String = conn
        .query_row(
            "SELECT name FROM chart_kind WHERE id = ?1",
            params![id],
            |row| row.get(0),
        )
        .map_err(|e| ChartKindError::RetrieveError(e.to_string()))?;

    Ok(name)
}

#[tauri::command]
pub fn list_chart_kinds() -> Result<Vec<String>, ChartKindError> {
    let conn = match get_connection() {
        Ok(conn) => conn,
        Err(e) => {
            return Err(ChartKindError::ConnectionError(
                e.to_string(),
                "all".to_string(),
            ))
        }
    };

    let mut stmt = conn
        .prepare("SELECT name FROM chart_kind;")
        .map_err(|e| ChartKindError::RetrieveError(e.to_string()))?;

    let names = stmt
        .query_map(params![], |row| row.get(0))
        .map_err(|e| ChartKindError::RetrieveError(e.to_string()))?;

    let mut chart_kinds = Vec::new();
    for name in names {
        chart_kinds.push(name.map_err(|e| ChartKindError::RetrieveError(e.to_string()))?);
    }

    Ok(chart_kinds)
}
