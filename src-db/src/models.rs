use crate::get_connection;
use sea_orm::entity::prelude::*;
use sea_orm::Set;
use std::io;
pub mod data_points;
pub mod projects;
pub mod settings;
pub mod theme;

pub async fn add_project(name: &str, description: &str) -> Result<(), io::Error> {
    let conn = get_connection().await;
    let project = projects::ActiveModel {
        name: Set(name.to_string()),
        description: Set(description.to_string()),
        ..Default::default()
    };
    let project = project.insert(&conn).await;
    println!("Added project: {:?}", project);
    Ok(())
}
