use crate::get_connection;
use sea_orm::entity::prelude::*;
use sea_orm::Set;
pub mod data_points;
pub mod projects;
pub mod settings;
pub mod theme;
use chrono::Utc;

pub async fn add_project(name: &str, description: &str) -> Result<(), DbErr> {
    println!("add_project function called");

    let conn = match get_connection().await {
        Ok(conn) => conn,
        Err(e) => {
            println!("Failed to get database connection: {:?}", e);
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
            println!("Added project: {:?}", project);
            Ok(())
        }
        Err(e) => {
            println!("Failed to insert project: {:?}", e);
            Err(e)
        }
    }
}
