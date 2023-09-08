use crate::get_connection;
use crate::models::{settings, theme};
use sea_orm::entity::prelude::*;
use sea_orm::Set;

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
