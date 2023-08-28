use sea_orm::DatabaseConnection;
use crate::{theme, settings};
use std::path::PathBuf;
use std::fs;

pub mod data_point;
pub mod project;
pub mod settings;
pub mod theme;

pub async fn populate(db: &DatabaseConnection) {
    // Add the default themes
    theme::Entity::insert(theme::ActiveModel {
        name: "auto".to_owned(),
        ..Default::default()
    })
    .exec(db)
    .await;

    theme::Entity::insert(theme::ActiveModel {
        name: "light".to_owned(),
        ..Default::default()
    })
    .exec(db)
    .await;

    theme::Entity::insert(theme::ActiveModel {
        name: "dark".to_owned(),
        ..Default::default()
    })
    .exec(db)
    .await;

    // Set the default theme to auto
    settings::Entity::insert(settings::ActiveModel {
        theme_selected: 1,
        ..Default::default()
    })
    .exec(db)
    .await;
}

pub async fn check_empty(db: &DatabaseConnection) -> bool {
    let theme_count: usize = theme::Entity::find().count(db).await.unwrap();
    theme_count == 0
}

pub fn db_string() -> String {
    // Assuming config_dir() returns Option<PathBuf>
    let base_dir = match config_dir() {
        Some(dir) => dir,
        None => panic!("Could not find config directory"),
    };
    println!("Base dir: {:?}", base_dir);

    // Make folder from config directory
    let mut db_dir = base_dir.clone();
    db_dir.push("ChartCharm");
    if !db_dir.exists() {
        fs::create_dir_all(&db_dir).expect("Failed to create directory");
    }
    println!("DB dir: {:?}", db_dir);

    // Concatenate DB string
    let mut db_path = db_dir;
    db_path.push("database.sqlite3");
    let db_string = format!("sqlite://{}", db_path.to_str().unwrap());
    println!("DB string: {}", db_string);

    db_string
}
