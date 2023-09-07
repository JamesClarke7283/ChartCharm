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

pub async fn update_theme(theme: &str) -> Result<(), DbErr> {
    println!("update_theme function called");

    let conn = match get_connection().await {
        Ok(conn) => conn,
        Err(e) => {
            println!("Failed to get database connection: {e:?}");
            return Err(DbErr::Custom(e.to_string()));
        }
    };

    println!("Got connection");
    // Query theme table for the id of the theme with the name of the theme passed in
    let theme_id = theme::Entity::find()
        .filter(theme::Column::Name.contains(theme))
        .one(&conn)
        .await?
        .unwrap()
        .id as u8;
    // Update the settings table's theme_selected column to the id of the theme with the name of the theme passed in
    let settings = settings::ActiveModel {
        id: Set(1),
        theme_selected: Set(theme_id),
        ..Default::default()
    };
    settings.save(&conn).await?;
    Ok(())
}

pub async fn query_theme() -> Result<String, DbErr> {
    println!("query_theme function called");

    let conn = match get_connection().await {
        Ok(conn) => conn,
        Err(e) => {
            println!("Failed to get database connection: {e:?}");
            return Err(DbErr::Custom(e.to_string()));
        }
    };

    println!("Got connection");
    // Query the settings table for the theme_selected column
    let theme_id = settings::Entity::find_by_id(1)
        .one(&conn)
        .await?
        .unwrap()
        .theme_selected as u8;
    // Query the theme table for the name of the theme with the id of the theme_selected column
    let theme_name = theme::Entity::find_by_id(theme_id)
        .one(&conn)
        .await?
        .unwrap()
        .name;
    Ok(theme_name)
}

pub async fn delete_project(project_id: i32) -> Result<(), DbErr> {
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
    let project = match projects::Entity::find_by_id(project_id).one(&conn).await? {
        Some(project) => project,
        None => {
            println!("Project with id {project_id} not found");
            return Err(DbErr::Custom("Project not found".to_string()));
        }
    };
    project.delete(&conn).await?;
    Ok(())
}
