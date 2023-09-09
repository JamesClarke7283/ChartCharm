use chartcharm_database::get_connection;
use chartcharm_database::models::{settings, theme};
use chartcharm_shared::settings::SettingsError;
use sea_orm::entity::prelude::*;
use sea_orm::{IntoActiveModel, Set};

#[tauri::command]
pub async fn update_theme(theme: &str) -> Result<(), SettingsError> {
    println!("update_theme function called");

    let conn = match get_connection().await {
        Ok(conn) => conn,
        Err(e) => {
            println!("Failed to get database connection: {e:?}");
            return Err(SettingsError::ConnectionError(
                e.to_string(),
                theme.to_string(),
            ));
        }
    };

    println!("Got connection");
    // Query theme table for the id of the theme with the name of the theme passed in
    let theme_id = match theme::Entity::find()
        .filter(theme::Column::Name.contains(theme))
        .one(&conn)
        .await
    {
        Ok(theme) => theme.unwrap().id,
        Err(e) => {
            println!("Failed to get theme: {e:?}");
            return Err(SettingsError::RetrieveError(e.to_string()));
        }
    };
    // Update the settings table's theme_selected column to the id of the theme with the name of the theme passed in
    let settings_item = match settings::Entity::find_by_id(1).one(&conn).await {
        Ok(settings_item) => settings_item.unwrap(),
        Err(e) => {
            println!("Failed to get settings: {e:?}");
            return Err(SettingsError::RetrieveError(e.to_string()));
        }
    };

    let mut settings_item = settings_item.into_active_model();

    settings_item.theme_selected = Set(theme_id);

    match settings_item.update(&conn).await {
        Ok(settings_item) => {
            println!("Updated settings: {settings_item:?}");
            return Ok(());
        }
        Err(e) => {
            println!("Failed to update settings: {e:?}");
            return Err(SettingsError::UpdateError(e.to_string()));
        }
    };
}

#[tauri::command]
pub async fn query_theme() -> Result<String, SettingsError> {
    println!("query_theme function called");

    let conn = match get_connection().await {
        Ok(conn) => conn,
        Err(e) => {
            println!("Failed to get database connection: {e:?}");
            return Err(SettingsError::ConnectionError(e.to_string(), String::new()));
        }
    };

    println!("Got connection");
    // Query the settings table for the theme_selected column
    let theme_id = match settings::Entity::find_by_id(1).one(&conn).await {
        Ok(settings_item) => settings_item.unwrap().theme_selected,
        Err(e) => {
            println!("Failed to get settings: {e:?}");
            return Err(SettingsError::RetrieveError(e.to_string()));
        }
    };
    // Query the theme table for the name of the theme with the id of the theme_selected column
    match theme::Entity::find_by_id(theme_id).one(&conn).await {
        Ok(theme) => Ok(theme.unwrap().name),
        Err(e) => {
            println!("Failed to get theme: {e:?}");
            return Err(SettingsError::RetrieveError(e.to_string()));
        }
    }
}
