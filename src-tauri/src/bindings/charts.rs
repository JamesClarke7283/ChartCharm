use chartcharm_database::{get_connection, rusqlite_to_string};
use chartcharm_shared::chart::{Chart, ChartError};
use chrono::prelude::*;
use rusqlite::{params, Connection, Result, Row};

pub fn create_charts_table() -> Result<(), ChartError> {
    let conn = match get_connection() {
        Ok(conn) => conn,
        Err(e) => {
            return Err(ChartError::ConnectionError(
                "N/A".to_string(),
                e.to_string(),
            ))
        }
    };
    conn.execute(
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
        params![],
    );
    Ok(())
}

#[tauri::command]
pub fn add_chart(
    name: String,
    description: String,
    project_id: u16,
    kind_id: u8,
) -> Result<(), ChartError> {
    let conn = match get_connection() {
        Ok(conn) => conn,
        Err(e) => {
            return Err(ChartError::ConnectionError(
                "N/A".to_string(),
                e.to_string(),
            ))
        }
    };
    conn.execute(
        "INSERT INTO charts (name, description, project, kind, created_at, updated_at) VALUES (?1, ?2, ?3, ?4, ?5, ?6);",
        params![
            name,
            description,
            project_id,
            kind_id,
            Utc::now().timestamp(),
            Utc::now().timestamp()
        ],
    );
    Ok(())
}

#[tauri::command]
pub fn list_charts() -> Result<Vec<Chart>, ChartError> {
    let conn = match get_connection() {
        Ok(conn) => conn,
        Err(e) => {
            return Err(ChartError::ConnectionError(
                "N/A".to_string(),
                e.to_string(),
            ))
        }
    };
    let mut stmt = match conn.prepare("SELECT * FROM charts;") {
        Ok(stmt) => stmt,
        Err(e) => {
            return Err(ChartError::RetrieveError(e.to_string()));
        }
    };
    let charts_iter = match stmt.query_map(params![], |row| {
        Ok(Chart {
            id: row.get(0)?,
            name: row.get(1)?,
            description: row.get(2)?,
            project: row.get(3)?,
            kind: row.get(4)?,
            created_at: Utc.timestamp(row.get(5)?, 0),
            updated_at: Utc.timestamp(row.get(6)?, 0),
        })
    }) {
        Ok(rows) => rows,
        Err(e) => {
            return Err(ChartError::RetrieveError(e.to_string()));
        }
    };

    let mut charts = Vec::new();
    for chart in charts_iter {
        match chart {
            Ok(chart) => charts.push(chart),
            Err(e) => {
                return Err(ChartError::RetrieveError(e.to_string()));
            }
        }
    }
    Ok(charts)
}

#[tauri::command]
pub fn query_chart(id: u16) -> Result<Chart, ChartError> {
    let conn = match get_connection() {
        Ok(conn) => conn,
        Err(e) => {
            return Err(ChartError::ConnectionError(
                "N/A".to_string(),
                e.to_string(),
            ))
        }
    };
    let mut stmt = match conn.prepare("SELECT * FROM charts WHERE id = ?1;") {
        Ok(stmt) => stmt,
        Err(e) => {
            return Err(ChartError::RetrieveError(e.to_string()));
        }
    };
    let chart = match stmt.query_row(params![id], |row| {
        Ok(Chart {
            id: row.get(0)?,
            name: row.get(1)?,
            description: row.get(2)?,
            project: row.get(3)?,
            kind: row.get(4)?,
            created_at: Utc.timestamp(row.get(5)?, 0),
            updated_at: Utc.timestamp(row.get(6)?, 0),
        })
    }) {
        Ok(chart) => chart,
        Err(e) => {
            return Err(ChartError::RetrieveError(e.to_string()));
        }
    };
    Ok(chart)
}

#[tauri::command]
pub fn delete_chart(id: u16) -> Result<(), ChartError> {
    let conn = match get_connection() {
        Ok(conn) => conn,
        Err(e) => {
            return Err(ChartError::ConnectionError(
                "N/A".to_string(),
                e.to_string(),
            ))
        }
    };
    conn.execute("DELETE FROM charts WHERE id = ?1;", params![id]);
    Ok(())
}

#[tauri::command]
pub fn update_chart(
    id: u16,
    name: String,
    description: String,
    project_id: u16,
    kind_id: u8,
) -> Result<(), ChartError> {
    let conn = match get_connection() {
        Ok(conn) => conn,
        Err(e) => {
            return Err(ChartError::ConnectionError(
                "N/A".to_string(),
                e.to_string(),
            ))
        }
    };
    conn.execute(
        "UPDATE charts SET name = ?1, description = ?2, project = ?3, kind = ?4, updated_at = ?5 WHERE id = ?6;",
        params![
            name,
            description,
            project_id,
            kind_id,
            Utc::now().timestamp(),
            id
        ],
    );
    Ok(())
}
