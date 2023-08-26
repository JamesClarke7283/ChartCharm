use sea_orm::{Database, DatabaseBackend, EntityTrait};
use models::theme::Entity as ThemeEntity;
use models::settings::Entity as SettingsEntity;
use dirs::config_dir;
use std::path::PathBuf;
use tokio::fs;

mod models {
    pub mod theme;
    pub mod settings;
    pub mod project;
    pub mod data_point;
}

pub async fn db_string() -> String {
    // Assuming config_dir() returns Option<PathBuf>
    let base_dir = config_dir().unwrap();

    // Make folder from config directory
    let mut db_dir = base_dir.clone();
    db_dir.push("ChartCharm");
    if !db_dir.exists() {
        fs::create_dir_all(&db_dir).await.expect("Failed to create directory");
    }

    // Concatenate DB string
    let mut db_path = db_dir;
    db_path.push("database.sqlite3");
    let db_string = format!("sqlite://{}", match db_path.to_str(){
        Some(path) => path,
        None => panic!("Could not convert path to string"),
    });

    db_string
}

pub async fn check_empty(db: &Database) -> bool {
    // Query the 'settings' table to see if it has any entries
    let settings_count = SettingsEntity::find().count(db).await.unwrap_or(0);
    settings_count == 0
}

pub async fn populate(db: &Database) {
    // Add the default themes
    let themes = vec![
        theme::ActiveModel {
            name: Set("auto".to_owned()),
            ..Default::default()
        },
        theme::ActiveModel {
            name: Set("light".to_owned()),
            ..Default::default()
        },
        theme::ActiveModel {
            name: Set("dark".to_owned()),
            ..Default::default()
        },
    ];
    
    // Insert themes
    for theme in themes {
        match theme.insert(db).await {
            Ok(_) => (),
            Err(e) => panic!("Could not insert theme: {}", e)
        }
    }
    
    // Set the default theme to 'auto' (assuming its id is 1)
    let setting = settings::ActiveModel {
        theme_selected: Set(1),
        ..Default::default()
    };
    
    // Insert setting
    match setting.insert(db).await {
        Ok(_) => (),
        Err(e) => panic!("Could not insert setting: {}", e)
    }
}

#[tokio::main]
async fn main() {
    let db_string = db_string().await;
    let db = match Database::connect(db_string)  {
        Ok(db) => db,
        Err(e) => panic!("Could not connect to database: {}", e)
    }.await;
    
    if check_empty(&db).await {
        println!("Database is empty, populating...");
        populate(&db).await;
        println!("Database populated.");
    } else {
        println!("Database is not empty.");
    }
}
