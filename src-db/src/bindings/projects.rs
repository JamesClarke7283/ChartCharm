use crate::get_connection;
use crate::models::projects;
use chartcharm_shared::Project;
use chrono::Utc;
use sea_orm::entity::prelude::*;
use sea_orm::Set;

/// # Errors
/// Database errors are returned as a `DbErr` enum.
pub async fn add_project(name: &str, description: &str) -> Result<(), DbErr> {
    println!("add_project function called");

    let conn = match get_connection().await {
        Ok(conn) => conn,
        Err(e) => {
            println!("Failed to get database connection: {e:?}");
            return Err(DbErr::Custom(e.to_string()));
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
            Err(e)
        }
    }
}

pub async fn list_projects() -> Result<Vec<Project>, DbErr> {
    println!("list_projects function called");

    let conn = match get_connection().await {
        Ok(conn) => conn,
        Err(e) => {
            println!("Failed to get database connection: {e:?}");
            return Err(DbErr::Custom(e.to_string()));
        }
    };

    println!("Got connection");

    let projects = projects::Entity::find().all(&conn).await?;
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

pub async fn delete_project(id: u16) -> Result<(), DbErr> {
    println!("delete_project function called");

    let conn = match get_connection().await {
        Ok(conn) => conn,
        Err(e) => {
            println!("Failed to get database connection: {e:?}");
            return Err(DbErr::Custom(e.to_string()));
        }
    };

    println!("Got connection");
    // Delete the project with the id passed in
    let project = match projects::Entity::find_by_id(id).one(&conn).await? {
        Some(project) => project,
        None => {
            println!("Project with id {id} not found");
            return Err(DbErr::Custom("Project not found".to_string()));
        }
    };
    project.delete(&conn).await?;
    Ok(())
}

pub async fn edit_project(id: u16, name: &str, description: &str) -> Result<(), DbErr> {
    println!("edit_project function called");

    let conn = match get_connection().await {
        Ok(conn) => conn,
        Err(e) => {
            println!("Failed to get database connection: {e:?}");
            return Err(DbErr::Custom(e.to_string()));
        }
    };

    println!("Got connection");

    let project = match projects::Entity::find_by_id(id).one(&conn).await? {
        Some(project) => project,
        None => {
            println!("Project with id {id} not found");
            return Err(DbErr::Custom("Project not found".to_string()));
        }
    };

    let mut project: projects::ActiveModel = project.into();
    project.name = Set(name.to_string());
    project.description = Set(description.to_string());
    project.updated_at = Set(Utc::now());
    project.update(&conn).await?;

    Ok(())
}
