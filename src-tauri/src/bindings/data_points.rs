use chartcharm_database::get_connection;
use chartcharm_shared::data_point::{DataPoint, DataPointError};
use chrono::prelude::*;
use rusqlite::{params, Result};

pub fn create_datapoints_table() -> Result<(), DataPointError> {
    let conn = match get_connection() {
        Ok(conn) => conn,
        Err(e) => {
            return Err(DataPointError::ConnectionError(
                e.to_string(),
                "N/A".to_string(),
            ))
        }
    };

    conn.execute(
        "CREATE TABLE IF NOT EXISTS data_points (
            id INTEGER PRIMARY KEY,
            project INTEGER NOT NULL,
            data REAL NOT NULL,
            created_at INTEGER NOT NULL,
            updated_at INTEGER NOT NULL,
            FOREIGN KEY (project) REFERENCES projects(id)
        );",
        params![],
    )
    .map_err(|e| DataPointError::CreateError(e.to_string()))?;
    Ok(())
}

#[tauri::command]
pub fn add_datapoint(project: u16, data: f32) -> Result<(), DataPointError> {
    let conn = match get_connection() {
        Ok(conn) => conn,
        Err(e) => {
            return Err(DataPointError::ConnectionError(
                e.to_string(),
                project.to_string(),
            ))
        }
    };

    conn.execute(
        "INSERT INTO data_points (project, data, created_at, updated_at) VALUES (?1, ?2, ?3, ?4);",
        params![
            project,
            data,
            Utc::now().timestamp(),
            Utc::now().timestamp()
        ],
    )
    .map_err(|e| DataPointError::InsertError(e.to_string()))?;
    Ok(())
}

#[tauri::command]
pub fn update_datapoint(id: u64, new_data: f32) -> Result<(), DataPointError> {
    let conn = match get_connection() {
        Ok(conn) => conn,
        Err(e) => {
            return Err(DataPointError::ConnectionError(
                e.to_string(),
                id.to_string(),
            ))
        }
    };

    conn.execute(
        "UPDATE data_points SET data = ?1, updated_at = ?2 WHERE id = ?3;",
        params![new_data, Utc::now().timestamp(), id],
    )
    .map_err(|e| DataPointError::UpdateError(e.to_string()))?;
    Ok(())
}

#[tauri::command]
pub fn delete_datapoint(id: u64) -> Result<(), DataPointError> {
    let conn = match get_connection() {
        Ok(conn) => conn,
        Err(e) => {
            return Err(DataPointError::ConnectionError(
                e.to_string(),
                id.to_string(),
            ))
        }
    };

    conn.execute("DELETE FROM data_points WHERE id = ?1;", params![id])
        .map_err(|e| DataPointError::DeleteError(e.to_string()))?;
    Ok(())
}

#[tauri::command]
pub fn list_datapoints(project: u16) -> Result<Vec<DataPoint>, DataPointError> {
    let conn = match get_connection() {
        Ok(conn) => conn,
        Err(e) => {
            return Err(DataPointError::ConnectionError(
                e.to_string(),
                project.to_string(),
            ))
        }
    };

    let mut stmt = conn
        .prepare("SELECT * FROM data_points WHERE project = ?1;")
        .map_err(|e| DataPointError::RetrieveError(e.to_string()))?;

    let data_point_rows = stmt
        .query_map(params![project], |row| {
            Ok(DataPoint {
                id: row.get(0)?,
                project: row.get(1)?,
                data: row.get(2)?,
                created_at: Utc.timestamp(row.get(3)?, 0),
                updated_at: Utc.timestamp(row.get(4)?, 0),
            })
        })
        .map_err(|e| DataPointError::RetrieveError(e.to_string()))?;

    let mut data_points = Vec::new();
    for data_point_row in data_point_rows {
        if let Ok(data_point) = data_point_row {
            data_points.push(data_point);
        }
    }
    Ok(data_points)
}

#[tauri::command]
pub fn query_datapoint(id: u64) -> Result<DataPoint, DataPointError> {
    let conn = get_connection()
        .map_err(|e| DataPointError::ConnectionError(e.to_string(), id.to_string()))?;

    let mut stmt = conn
        .prepare("SELECT * FROM data_points WHERE id = ?1;")
        .map_err(|e| DataPointError::RetrieveError(e.to_string()))?;

    let datapoint = match stmt.query_row(params![id], |row| {
        Ok(DataPoint {
            id: row.get(0)?,
            project: row.get(1)?,
            data: row.get(2)?,
            created_at: Utc.timestamp(row.get(3)?, 0),
            updated_at: Utc.timestamp(row.get(4)?, 0),
        })
    }) {
        Ok(datapoint) => datapoint,
        Err(e) => return Err(DataPointError::RetrieveError(e.to_string())),
    };

    Ok(datapoint)
}
