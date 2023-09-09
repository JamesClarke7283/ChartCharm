use chartcharm_database::get_connection;
use chartcharm_database::models::projects;
use chartcharm_shared::{Project, ProjectError};
use chrono::Utc;
use sea_orm::entity::prelude::*;
use sea_orm::Set;

/// # Errors
/// Database errors are returned as a `DbErr` enum.
#[tauri::command]
pub async fn add_project(name: &str, description: &str) -> Result<(), ProjectError> {
    println!("add_project function called");

    let conn = match get_connection().await {
        Ok(conn) => conn,
        Err(e) => {
            println!("Failed to get database connection: {e:?}");
            return Err(ProjectError::ConnectionError(
                e.to_string(),
                name.to_string(),
            ));
        }
    };

    println!("Got connection");

    let project = projects::ActiveModel {
        name: Set(name.to_string()),
        description: Set(description.to_string()),
        created_at: Set(Utc::now()),
        updated_at: Set(Utc::now()),
        ..Default::default()
    };

    match project.insert(&conn).await {
        Ok(project) => {
            println!("Added project: {project:?}");
            Ok(())
        }
        Err(e) => {
            println!("Failed to insert project: {e:?}");
            Err(ProjectError::InsertError(e.to_string()))
        }
    }
}

#[tauri::command]
pub async fn list_projects() -> Result<Vec<Project>, ProjectError> {
    println!("list_projects function called");

    let conn = match get_connection().await {
        Ok(conn) => conn,
        Err(e) => {
            println!("Failed to get database connection: {e:?}");
            return Err(ProjectError::ConnectionError(
                e.to_string(),
                "all".to_string(),
            ));
        }
    };

    println!("Got connection");

    let projects = match projects::Entity::find().all(&conn).await {
        Ok(projects) => projects,
        Err(e) => {
            println!("Failed to retrieve projects: {e:?}");
            return Err(ProjectError::RetrieveError(e.to_string()));
        }
    };
    let new_projects = projects
        .into_iter()
        .map(|project| Project {
            id: project.id,
            name: project.name,
            description: project.description,
            created_at: project.created_at,
            updated_at: project.updated_at,
        })
        .collect();

    println!("Retrieved projects");

    Ok(new_projects)
}

#[tauri::command]
pub async fn delete_project(id: u16) -> Result<(), ProjectError> {
    println!("delete_project function called");

    let conn = match get_connection().await {
        Ok(conn) => conn,
        Err(e) => {
            println!("Failed to get database connection: {e:?}");
            return Err(ProjectError::ConnectionError(e.to_string(), id.to_string()));
        }
    };

    println!("Got connection");
    // Delete the project with the id passed in
    let project = match projects::Entity::find_by_id(id).one(&conn).await {
        Ok(project) => match project {
            Some(project) => project,
            None => {
                println!("Project with id {id} not found");
                return Err(ProjectError::RetrieveError(id.to_string()));
            }
        },
        Err(e) => {
            println!("Failed to retrieve project: {e:?}");
            return Err(ProjectError::RetrieveError(e.to_string()));
        }
    };
    match project.delete(&conn).await {
        Ok(project) => {
            println!("Deleted project: {project:?}");
            return Ok(());
        }
        Err(e) => {
            println!("Failed to delete project: {e:?}");
            return Err(ProjectError::DeleteError(e.to_string()));
        }
    };
}

#[tauri::command]
pub async fn edit_project(id: u16, name: &str, description: &str) -> Result<(), ProjectError> {
    println!("edit_project function called");

    let conn = match get_connection().await {
        Ok(conn) => conn,
        Err(e) => {
            println!("Failed to get database connection: {e:?}");
            return Err(ProjectError::ConnectionError(e.to_string(), id.to_string()));
        }
    };

    println!("Got connection");

    let project = match projects::Entity::find_by_id(id).one(&conn).await {
        Ok(project) => match project {
            Some(project) => project,
            None => {
                println!("Project with id {id} not found");
                return Err(ProjectError::RetrieveError(id.to_string()));
            }
        },
        Err(e) => {
            println!("Failed to retrieve project: {e:?}");
            return Err(ProjectError::RetrieveError(e.to_string()));
        }
    };
    if project.name.is_empty() {
        println!("Project with id {id} not found");
        return Err(ProjectError::RetrieveError(id.to_string()));
    } else {
        let mut project: projects::ActiveModel = project.into();
        project.name = Set(name.to_string());
        project.description = Set(description.to_string());
        project.updated_at = Set(Utc::now());
        match project.update(&conn).await {
            Ok(project) => {
                println!("Updated project: {project:?}");
                return Ok(());
            }
            Err(e) => {
                println!("Failed to update project: {e:?}");
                return Err(ProjectError::UpdateError(e.to_string()));
            }
        };
    }
}

#[tauri::command]
pub async fn query_project(id: u16) -> Result<Project, ProjectError> {
    println!("query_project function called");

    let conn = match get_connection().await {
        Ok(conn) => conn,
        Err(e) => {
            println!("Failed to get database connection: {e:?}");
            return Err(ProjectError::ConnectionError(e.to_string(), id.to_string()));
        }
    };

    println!("Got connection");

    match projects::Entity::find_by_id(id).one(&conn).await {
        Ok(project) => match project {
            Some(project) => Ok(Project {
                id: project.id,
                name: project.name,
                description: project.description,
                created_at: project.created_at,
                updated_at: project.updated_at,
            }),
            None => {
                println!("Project with id {id} not found");
                return Err(ProjectError::RetrieveError(id.to_string()));
            }
        },
        Err(e) => {
            println!("Failed to retrieve project: {e:?}");
            return Err(ProjectError::RetrieveError(e.to_string()));
        }
    }
}
