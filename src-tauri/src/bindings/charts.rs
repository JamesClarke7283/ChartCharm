use chartcharm_database::get_connection;
use chartcharm_shared::chart::{Chart, ChartError};
use chrono::prelude::*;

pub fn create_charts_table() -> Result<(), ChartError> {
    let mut db = get_connection()
        .map_err(|e| ChartError::ConnectionError(e.to_string(), "all".to_string()))?;

    let mut stmt = db
        .prepare(
            "CREATE TABLE IF NOT EXISTS charts (
        id INTEGER PRIMARY KEY,
        name TEXT NOT NULL,
        description TEXT NOT NULL,
        project INTEGER NOT NULL,
        kind INTEGER NOT NULL,
        created_at INTEGER NOT NULL,
        updated_at INTEGER NOT NULL,
        FOREIGN KEY (project) REFERENCES projects(id),
        FOREIGN KEY (kind) REFERENCES chart_kind(id)
    );",
        )
        .unwrap();

    stmt.execute().map_err(|e| ChartError::CreateError)?;

    Ok(())
}

#[tauri::command]
pub fn add_chart(
    name: String,
    description: String,
    project_id: u16,
    kind_id: u8,
) -> Result<(), ChartError> {
    let mut db = get_connection()
        .map_err(|e| ChartError::ConnectionError(e.to_string(), name.to_string()))?;

    let created_at = Utc::now().timestamp();
    let updated_at = created_at;
    let insert_sql = format!(
        "INSERT INTO charts (name, description, project, kind, created_at, updated_at) VALUES ('{}', '{}', {}, {}, {}, {});",
        name, description, project_id, kind_id, created_at, updated_at
    );

    let mut stmt = db
        .prepare(&insert_sql)
        .map_err(|e| ChartError::InsertError(e.to_string()))?;

    stmt.execute()
        .map_err(|e| ChartError::InsertError(e.to_string()))?;

    Ok(())
}

// list_charts function
#[tauri::command]
pub fn list_charts() -> Result<Vec<Chart>, ChartError> {
    let mut db = get_connection()
        .map_err(|e| ChartError::ConnectionError(e.to_string(), "all".to_string()))?;

    let query_sql = "SELECT * FROM charts;";
    let mut stmt = db.prepare(&query_sql).unwrap();
    let mut rows = stmt.execute().unwrap();

    let mut charts = Vec::new();
    while let Some(row) = rows.next_row().unwrap() {
        let columns = row.parse().unwrap();
        let id: u16 = columns.get(0).as_integer().unwrap() as u16;
        let name: String = columns.get(1).as_string().unwrap().to_string();
        let description: String = columns.get(2).as_string().unwrap().to_string();
        let project: u16 = columns.get(3).as_integer().unwrap() as u16;
        let kind: u8 = columns.get(4).as_integer().unwrap() as u8;
        let created_at: i64 = columns.get(5).as_integer().unwrap();
        let updated_at: i64 = columns.get(6).as_integer().unwrap();

        let chart = Chart {
            id,
            name,
            description,
            project,
            kind,
            created_at: Utc.timestamp(created_at, 0),
            updated_at: Utc.timestamp(updated_at, 0),
        };

        charts.push(chart);
    }

    Ok(charts)
}

// query_chart function
#[tauri::command]
pub fn query_chart(id: u16) -> Result<Chart, ChartError> {
    let mut db =
        get_connection().map_err(|e| ChartError::ConnectionError(e.to_string(), id.to_string()))?;

    let query_sql = format!("SELECT * FROM charts WHERE id = {};", id);
    let mut stmt = db.prepare(&query_sql).unwrap();
    let mut rows = stmt.execute().unwrap();
    let row = rows.next_row().unwrap().unwrap();
    let columns = row.parse().unwrap();

    let id: u16 = columns.get(0).as_integer().unwrap() as u16;
    let name: String = columns.get(1).as_string().unwrap().to_string();
    let description: String = columns.get(2).as_string().unwrap().to_string();
    let project: u16 = columns.get(3).as_integer().unwrap() as u16;
    let kind: u8 = columns.get(4).as_integer().unwrap() as u8;
    let created_at: i64 = columns.get(5).as_integer().unwrap();
    let updated_at: i64 = columns.get(6).as_integer().unwrap();

    let chart = Chart {
        id,
        name,
        description,
        project,
        kind,
        created_at: Utc.timestamp(created_at, 0),
        updated_at: Utc.timestamp(updated_at, 0),
    };

    Ok(chart)
}

// delete_chart function
#[tauri::command]
pub fn delete_chart(id: u16) -> Result<(), ChartError> {
    let mut db =
        get_connection().map_err(|e| ChartError::ConnectionError(e.to_string(), id.to_string()))?;

    let delete_sql = format!("DELETE FROM charts WHERE id = {};", id);
    let mut stmt = db.prepare(&delete_sql).unwrap();
    stmt.execute()
        .map_err(|e| ChartError::DeleteError(e.to_string()))?;

    Ok(())
}

// update_chart function
#[tauri::command]
pub fn update_chart(
    id: u16,
    name: String,
    description: String,
    project_id: u16,
    kind_id: u8,
) -> Result<(), ChartError> {
    let mut db =
        get_connection().map_err(|e| ChartError::ConnectionError(e.to_string(), id.to_string()))?;

    let updated_at = Utc::now().timestamp();
    let update_sql = format!(
        "UPDATE charts SET name = '{}', description = '{}', project = {}, kind = {}, updated_at = {} WHERE id = {};",
        name, description, project_id, kind_id, updated_at, id
    );

    let mut stmt = db.prepare(&update_sql).unwrap();
    stmt.execute()
        .map_err(|e| ChartError::UpdateError(e.to_string()))?;

    Ok(())
}
