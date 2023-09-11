use chartcharm_database::get_connection;
use chartcharm_shared::project::{Project, ProjectError};
use chrono::prelude::*;

pub fn create_projects_table() -> Result<(), ProjectError> {
    let mut db = get_connection()
        .map_err(|e| ProjectError::ConnectionError("N/A".to_string(), e.to_string()))?;
    let create_table_sql = "CREATE TABLE projects IF NOT EXISTS(
            id INTEGER PRIMARY KEY,
            name TEXT NOT NULL,
            description TEXT NOT NULL,
            created_at INTEGER NOT NULL,
            updated_at INTEGER NOT NULL
        );";
    let mut stmt = db
        .prepare(create_table_sql)
        .map_err(|e| ProjectError::CreateError(e.to_string()))?;

    stmt.execute()
        .map_err(|e| ProjectError::CreateError(e.to_string()))?;
    Ok(())
}

#[tauri::command]
pub fn add_project(name: &str, description: &str) -> Result<(), ProjectError> {
    let mut db = get_connection()
        .map_err(|e| ProjectError::ConnectionError(e.to_string(), name.to_string()))?;

    let mut stmt = db
        .prepare(&format!(
            "INSERT INTO projects (name, description, created_at, updated_at) VALUES ('{}', '{}', {}, {});",
            name,
            description,
            Utc::now().timestamp(),
            Utc::now().timestamp()
        ))
        .map_err(|e| ProjectError::InsertError(e.to_string()))?;

    stmt.execute()
        .map_err(|e| ProjectError::InsertError(e.to_string()))?;
    Ok(())
}

#[tauri::command]
pub fn list_projects() -> Result<Vec<Project>, ProjectError> {
    let mut db = get_connection()
        .map_err(|e| ProjectError::ConnectionError(e.to_string(), "all".to_string()))?;

    let mut stmt = db
        .prepare("SELECT * FROM projects;")
        .map_err(|e| ProjectError::RetrieveError(e.to_string()))?;

    let mut rows = stmt
        .execute()
        .map_err(|e| ProjectError::RetrieveError(e.to_string()))?;

    let mut projects: Vec<Project> = Vec::new();

    while let Some(row) = rows.next_row().unwrap() {
        let columns = row.parse().unwrap();
        let id = columns.get(0).as_string().unwrap().parse::<u16>().unwrap();
        let name = columns.get(1).as_string().unwrap();
        let description = columns.get(2).as_string().unwrap();
        let created_at = NaiveDateTime::from_timestamp(
            columns.get(3).as_string().unwrap().parse::<i64>().unwrap(),
            0,
        );
        let updated_at = NaiveDateTime::from_timestamp(
            columns.get(4).as_string().unwrap().parse::<i64>().unwrap(),
            0,
        );

        projects.push(Project {
            id,
            name,
            description,
            created_at: DateTime::<Utc>::from_utc(created_at, Utc),
            updated_at: DateTime::<Utc>::from_utc(updated_at, Utc),
        });
    }

    Ok(projects)
}

#[tauri::command]
pub fn delete_project(id: u16) -> Result<(), ProjectError> {
    let mut db = get_connection()
        .map_err(|e| ProjectError::ConnectionError(e.to_string(), id.to_string()))?;

    let mut stmt = db
        .prepare(&format!("DELETE FROM projects WHERE id = {};", id))
        .map_err(|e| ProjectError::DeleteError(e.to_string()))?;

    stmt.execute()
        .map_err(|e| ProjectError::DeleteError(e.to_string()))?;

    Ok(())
}

#[tauri::command]
pub fn edit_project(id: u16, name: &str, description: &str) -> Result<(), ProjectError> {
    let mut db = get_connection()
        .map_err(|e| ProjectError::ConnectionError(e.to_string(), id.to_string()))?;

    let mut stmt = db
        .prepare(&format!(
            "UPDATE projects SET name = '{}', description = '{}', updated_at = {} WHERE id = {};",
            name,
            description,
            Utc::now().timestamp(),
            id
        ))
        .map_err(|e| ProjectError::UpdateError(e.to_string()))?;

    stmt.execute()
        .map_err(|e| ProjectError::UpdateError(e.to_string()))?;

    Ok(())
}

#[tauri::command]
pub fn query_project(id: u16) -> Result<Project, ProjectError> {
    let mut db = get_connection()
        .map_err(|e| ProjectError::ConnectionError(e.to_string(), id.to_string()))?;

    let mut stmt = db
        .prepare(&format!("SELECT * FROM projects WHERE id = {};", id))
        .map_err(|e| ProjectError::RetrieveError(e.to_string()))?;

    let mut rows = match stmt.execute() {
        Ok(rows) => rows,
        Err(e) => {
            println!("Failed to execute project query statement: {:?}", e);
            return Err(ProjectError::RetrieveError(e.to_string()));
        }
    };

    let row = rows.next_row().unwrap().unwrap();
    let columns = row.parse().unwrap();

    let id = columns.get(0).as_string().unwrap().parse::<u16>().unwrap();
    let name = columns.get(1).as_string().unwrap();
    let description = columns.get(2).as_string().unwrap();
    let created_at = NaiveDateTime::from_timestamp(
        columns.get(3).as_string().unwrap().parse::<i64>().unwrap(),
        0,
    );
    let updated_at = NaiveDateTime::from_timestamp(
        columns.get(4).as_string().unwrap().parse::<i64>().unwrap(),
        0,
    );

    Ok(Project {
        id,
        name,
        description,
        created_at: DateTime::<Utc>::from_utc(created_at, Utc),
        updated_at: DateTime::<Utc>::from_utc(updated_at, Utc),
    })
}
