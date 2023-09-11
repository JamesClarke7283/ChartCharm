use chartcharm_database::get_connection;
use chartcharm_shared::chart_kind::{ChartKind, ChartKindError};
use chrono::prelude::*;

pub fn populate_chartkind_table() -> Result<(), ChartKindError> {
    let mut db = get_connection()
        .map_err(|e| ChartKindError::ConnectionError("N/A".to_string(), e.to_string()))?;
    let mut stmt = db
        .prepare("INSERT INTO chart_kind (name) VALUES ('line');")
        .map_err(|e| ChartKindError::InsertError(e.to_string()))?;

    stmt.execute()
        .map_err(|e| ChartKindError::InsertError(e.to_string()))?;
    Ok(())
}

pub fn create_chartkind_table() -> Result<(), ChartKindError> {
    let mut db = get_connection()
        .map_err(|e| ChartKindError::ConnectionError(e.to_string(), "all".to_string()))?;

    let mut stmt = db
        .prepare(
            "CREATE TABLE IF NOT EXISTS chart_kind (
            id INTEGER PRIMARY KEY,
            name TEXT NOT NULL
        );",
        )
        .unwrap();

    stmt.execute().map_err(|e| ChartKindError::CreateError)?;

    Ok(())
}

#[tauri::command]
pub fn query_chart_kind(id: u8) -> Result<String, ChartKindError> {
    let mut db = get_connection()
        .map_err(|e| ChartKindError::ConnectionError(e.to_string(), id.to_string()))?;

    let query_sql = format!("SELECT * FROM chart_kind WHERE id = {};", id);
    let mut stmt = db.prepare(&query_sql).unwrap();
    let mut rows = stmt.execute().unwrap();
    let row = rows.next_row().unwrap().unwrap();
    let columns = row.parse().unwrap();
    let name: String = columns.get(1).as_string().unwrap().to_string();

    Ok(name)
}

#[tauri::command]
pub fn list_chart_kinds() -> Result<Vec<String>, ChartKindError> {
    let mut db = get_connection()
        .map_err(|e| ChartKindError::ConnectionError(e.to_string(), "all".to_string()))?;

    let query_sql = "SELECT * FROM chart_kind;";
    let mut stmt = db.prepare(&query_sql).unwrap();
    let mut rows = stmt.execute().unwrap();

    let mut chart_kinds = Vec::new();
    while let Some(row) = rows.next_row().unwrap() {
        let columns = row.parse().unwrap();
        let name: String = columns.get(1).as_string().unwrap().to_string();
        chart_kinds.push(name);
    }

    Ok(chart_kinds)
}
