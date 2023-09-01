#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use chartcharm_database;
use tauri::Builder;

#[tauri::command]
async fn add_project(name: &str, description: &str) -> Result<String, String> {
    let name = name.to_string();
    let description = description.to_string();

    println!(
        "Added Project: '{}' with description '{}'",
        name, description
    );

    match chartcharm_database::models::add_project(&name, &description).await {
        Ok(_) => {
            println!("Successfully added project");
            Ok("Successfully added project".to_string())
        }
        Err(e) => {
            eprintln!("Failed to add project: {}", e);
            Err(format!("Failed to add project: {}", e))
        }
    }
}

#[async_std::main]
async fn main() {
    println!("Starting Tauri application");
    if let Err(e) = chartcharm_database::init_db().await {
        eprintln!("Failed to initialize the database: {}", e);
    }

    Builder::default()
        .invoke_handler(tauri::generate_handler![add_project])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
