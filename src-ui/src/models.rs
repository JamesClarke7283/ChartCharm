use dirs::config_dir;
use sea_orm::{ActiveValue, DatabaseConnection, EntityTrait};
use std::fs;

pub mod data_point;
pub mod project;
pub mod settings;
pub mod theme;

pub async fn populate(db: &DatabaseConnection) {
    // Add the default themes
    theme::Entity::insert(theme::ActiveModel {
        name: ActiveValue::Set("auto".to_owned()),
        ..Default::default()
    })
    .exec(db)
    .await
    .unwrap();

    theme::Entity::insert(theme::ActiveModel {
        name: ActiveValue::Set("light".to_owned()),
        ..Default::default()
    })
    .exec(db)
    .await
    .unwrap();

    theme::Entity::insert(theme::ActiveModel {
        name: ActiveValue::Set("dark".to_owned()),
        ..Default::default()
    })
    .exec(db)
    .await
    .unwrap();

    // Set the default theme to auto
    settings::Entity::insert(settings::ActiveModel {
        theme_selected: ActiveValue::Set(1),
        ..Default::default()
    })
    .exec(db)
    .await
    .unwrap();
}

pub async fn check_empty(db: &DatabaseConnection) -> bool {
    let themes = theme::Entity::find().all(db).await.unwrap();
    let theme_count = themes.len();

    theme_count == 0
}

pub fn db_string() -> String {
    let base_dir = config_dir().expect("Could not find config directory");
    println!("Base dir: {:?}", base_dir);

    let mut db_dir = base_dir.clone();
    db_dir.push("ChartCharm");
    if !db_dir.exists() {
        fs::create_dir_all(&db_dir).expect("Failed to create directory");
    }
    println!("DB dir: {:?}", db_dir);

    let mut db_path = db_dir;
    db_path.push("database.sqlite3");
    let db_string = format!("sqlite://{}", db_path.to_str().unwrap());
    println!("DB string: {}", db_string);

    db_string
}
