use chartcharm_database::{get_connection, is_db_populated, rusqlite_to_string};
use chartcharm_shared::project::{Project, ProjectError};
use chrono::prelude::*;
use rusqlite::{params, Result};

pub fn create_projects_table() -> Result<(), ProjectError> {
    let conn = match get_connection() {
        Ok(conn) => conn,
        Err(e) => {
            return Err(ProjectError::ConnectionError(
                "N/A".to_string(),
                e.to_string(),
            ))
        }
    };
    if is_db_populated() {
        return Ok(());
    } else {
        conn.execute(
            "CREATE TABLE IF NOT EXISTS projects (
            id INTEGER PRIMARY KEY,
            name TEXT NOT NULL,
            description TEXT NOT NULL,
            created_at INTEGER NOT NULL,
            updated_at INTEGER NOT NULL
        );",
            params![],
        )
        .map_err(|e| ProjectError::CreateError(e.to_string()))?;
        Ok(())
    }
}

#[tauri::command]
pub fn add_project(name: &str, description: &str) -> Result<(), ProjectError> {
    let conn = match get_connection() {
        Ok(conn) => conn,
        Err(e) => {
            return Err(ProjectError::ConnectionError(
                e.to_string(),
                name.to_string(),
            ))
        }
    };

    conn.execute(
        "INSERT INTO projects (name, description, created_at, updated_at) VALUES (?1, ?2, ?3, ?4);",
        params![
            name,
            description,
            Utc::now().timestamp(),
            Utc::now().timestamp()
        ],
    )
    .map_err(|e| ProjectError::InsertError(e.to_string()))?;
    Ok(())
}

#[tauri::command]
pub fn delete_project(id: u16) -> Result<(), ProjectError> {
    let conn = match get_connection() {
        Ok(conn) => conn,
        Err(e) => return Err(ProjectError::ConnectionError(e.to_string(), id.to_string())),
    };

    conn.execute("DELETE FROM projects WHERE id = ?1;", params![id])
        .map_err(|e| ProjectError::DeleteError(e.to_string()))?;
    Ok(())
}

#[tauri::command]
pub fn edit_project(id: u16, name: &str, description: &str) -> Result<(), ProjectError> {
    let conn = match get_connection() {
        Ok(conn) => conn,
        Err(e) => return Err(ProjectError::ConnectionError(e.to_string(), id.to_string())),
    };

    conn.execute(
        "UPDATE projects SET name = ?1, description = ?2, updated_at = ?3 WHERE id = ?4;",
        params![name, description, Utc::now().timestamp(), id],
    )
    .map_err(|e| ProjectError::UpdateError(e.to_string()))?;
    Ok(())
}

#[tauri::command]
pub fn list_projects() -> Result<Vec<Project>, ProjectError> {
    let conn = match get_connection() {
        Ok(conn) => conn,
        Err(e) => {
            return Err(ProjectError::ConnectionError(
                e.to_string(),
                "all".to_string(),
            ))
        }
    };

    let mut stmt = conn
        .prepare("SELECT * FROM projects;")
        .map_err(|e| ProjectError::RetrieveError(e.to_string()))?;

    let mut rows = stmt
        .query(params![])
        .map_err(|e| ProjectError::RetrieveError(e.to_string()))?;

    let mut projects: Vec<Project> = Vec::new();
    while let Some(row) = rows
        .next()
        .map_err(|e| ProjectError::RetrieveError(e.to_string()))?
    {
        let id: u16 = match row.get(0) {
            Ok(id) => id,
            Err(e) => return Err(ProjectError::RetrieveError(rusqlite_to_string(e))),
        };
        let name: String = match row.get(1) {
            Ok(name) => name,
            Err(e) => return Err(ProjectError::RetrieveError(rusqlite_to_string(e))),
        };
        let description: String = match row.get(2) {
            Ok(description) => description,
            Err(e) => return Err(ProjectError::RetrieveError(rusqlite_to_string(e))),
        };
        let created_at: i64 = match row.get(3) {
            Ok(created_at) => created_at,
            Err(e) => return Err(ProjectError::RetrieveError(rusqlite_to_string(e))),
        };
        let updated_at: i64 = match row.get(4) {
            Ok(updated_at) => updated_at,
            Err(e) => return Err(ProjectError::RetrieveError(rusqlite_to_string(e))),
        };
        projects.push(Project {
            id,
            name,
            description,
            created_at: DateTime::<Utc>::from_utc(
                NaiveDateTime::from_timestamp(created_at, 0),
                Utc,
            ),
            updated_at: DateTime::<Utc>::from_utc(
                NaiveDateTime::from_timestamp(updated_at, 0),
                Utc,
            ),
        });
    }

    Ok(projects)
}

#[tauri::command]
pub fn query_project(id: u16) -> Result<Project, ProjectError> {
    let conn = get_connection()
        .map_err(|e| ProjectError::ConnectionError(e.to_string(), id.to_string()))?;

    let mut stmt = conn
        .prepare("SELECT * FROM projects WHERE id = ?1;")
        .map_err(|e| ProjectError::RetrieveError(e.to_string()))?;

    let project = match stmt.query_row(params![id], |row| {
        Ok(Project {
            id: row.get(0)?,
            name: row.get(1)?,
            description: row.get(2)?,
            created_at: DateTime::<Utc>::from_utc(
                NaiveDateTime::from_timestamp(row.get(3)?, 0),
                Utc,
            ),
            updated_at: DateTime::<Utc>::from_utc(
                NaiveDateTime::from_timestamp(row.get(4)?, 0),
                Utc,
            ),
        })
    }) {
        Ok(project) => project,
        Err(e) => return Err(ProjectError::RetrieveError(e.to_string())),
    };

    Ok(project)
}
