use crate::get_connection;
use sea_orm::entity::prelude::*;
use sea_orm::Set;
pub mod data_points;
pub mod projects;
pub mod settings;
pub mod theme;
use chartcharm_shared::Project;
use chrono::Utc;

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
