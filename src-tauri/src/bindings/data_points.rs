use chartcharm_database::get_connection;
use chartcharm_shared::data_point::{DataPoint, DataPointError};
use chrono::prelude::*;

pub fn create_datapoints_table() -> Result<(), DataPointError> {
    let mut db = get_connection()
        .map_err(|e| DataPointError::ConnectionError("N/A".to_string(), e.to_string()))?;
    let create_table_sql = "CREATE TABLE IF NOT EXISTS data_points (
        id INTEGER PRIMARY KEY,
        project INTEGER NOT NULL,
        data REAL NOT NULL,
        created_at INTEGER NOT NULL,
        updated_at INTEGER NOT NULL,
        FOREIGN KEY (project) REFERENCES projects(id)
    );";
    let mut stmt = db
        .prepare(create_table_sql)
        .map_err(|e| DataPointError::CreateError(e.to_string()))?;

    stmt.execute()
        .map_err(|e| DataPointError::CreateError(e.to_string()))?;
    Ok(())
}

#[tauri::command]
pub fn add_datapoint(project: u16, data: f32) -> Result<(), DataPointError> {
    let mut db = get_connection()
        .map_err(|e| DataPointError::ConnectionError(e.to_string(), project.to_string()))?;

    let created_at = Utc::now().timestamp();
    let updated_at = created_at;

    let insert_sql =
        format!("INSERT INTO data_points (project, data, created_at, updated_at) VALUES ({project}, {data}, {created_at}, {updated_at});");
    let mut stmt = db.prepare(&insert_sql).unwrap();

    stmt.execute()
        .map_err(|e| DataPointError::InsertError(e.to_string()))?;

    Ok(())
}

#[tauri::command]
pub fn list_datapoints(project: u16) -> Result<Vec<DataPoint>, DataPointError> {
    let mut db = get_connection()
        .map_err(|e| DataPointError::ConnectionError(e.to_string(), project.to_string()))?;

    let query_sql = format!("SELECT * FROM data_points WHERE project = {project}");
    let mut stmt = db.prepare(&query_sql).unwrap();
    let mut rows = stmt.execute().unwrap();

    let mut data_points = Vec::new();

    while let Some(row) = rows.next_row().unwrap() {
        let columns = row.parse().unwrap();
        let id: u64 = columns.get(0).as_integer().unwrap() as u64;
        let project: u16 = columns.get(1).as_integer().unwrap() as u16;
        let data: f32 = columns.get(2).as_float().unwrap() as f32;
        let created_at: i64 = columns.get(3).as_integer().unwrap();
        let updated_at: i64 = columns.get(4).as_integer().unwrap();

        let data_point = DataPoint {
            id,
            project,
            data,
            created_at: Utc.timestamp(created_at, 0),
            updated_at: Utc.timestamp(updated_at, 0),
        };

        data_points.push(data_point);
    }

    Ok(data_points)
}

#[tauri::command]
pub fn query_datapoint(id: u64) -> Result<DataPoint, DataPointError> {
    let mut db = get_connection()
        .map_err(|e| DataPointError::ConnectionError(e.to_string(), id.to_string()))?;

    let mut stmt = db
        .prepare(&format!(
            "SELECT * FROM data_points WHERE id = {}",
            id.to_string()
        ))
        .unwrap();

    let mut rows = stmt.execute().unwrap();
    let row = rows.next_row().unwrap().unwrap();
    let columns = row.parse().unwrap();

    let id: u64 = columns.get(0).as_integer().unwrap() as u64;
    let project: u16 = columns.get(1).as_integer().unwrap() as u16;
    let data: f32 = columns.get(2).as_float().unwrap() as f32;
    let created_at: i64 = columns.get(3).as_integer().unwrap();
    let updated_at: i64 = columns.get(4).as_integer().unwrap();

    let data_point = DataPoint {
        id,
        project,
        data,
        created_at: Utc.timestamp(created_at, 0),
        updated_at: Utc.timestamp(updated_at, 0),
    };

    Ok(data_point)
}

#[tauri::command]
pub fn update_datapoint(id: u64, new_data: f32) -> Result<(), DataPointError> {
    let mut db = get_connection()
        .map_err(|e| DataPointError::ConnectionError(e.to_string(), id.to_string()))?;

    let updated_at = Utc::now().timestamp();

    let mut stmt = db
        .prepare(&format!(
            "UPDATE data_points SET data = {new_data}, updated_at = {updated_at} WHERE id = {id}"
        ))
        .unwrap();

    stmt.execute()
        .map_err(|e| DataPointError::UpdateError(e.to_string()))?;

    Ok(())
}

#[tauri::command]
pub fn delete_datapoint(id: u64) -> Result<(), DataPointError> {
    let mut db = get_connection()
        .map_err(|e| DataPointError::ConnectionError(e.to_string(), id.to_string()))?;

    let mut stmt = db
        .prepare(&format!("DELETE FROM data_points WHERE id = {id}"))
        .unwrap();

    stmt.execute()
        .map_err(|e| DataPointError::DeleteError(e.to_string()))?;

    Ok(())
}
