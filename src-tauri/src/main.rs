#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use chartcharm_database::init_db;
use tauri::Builder;

#[tauri::command]
fn add_project(name: &str, description: &str) {
    println!(
        "Added Project: '{}' with description '{}'",
        name, description
    );
}

#[async_std::main]
async fn main() {
    println!("Starting Tauri application");
    if let Err(e) = init_db().await {
        eprintln!("Failed to initialize the database: {}", e);
    }

    Builder::default()
        .invoke_handler(tauri::generate_handler![add_project])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
