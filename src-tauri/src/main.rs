#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use chartcharm_database::models;
use chartcharm_shared::Project;
use tauri::Builder;

#[tauri::command]
async fn add_project(name: &str, description: &str) -> Result<(), String> {
    let name = name.to_string();
    let description = description.to_string();

    println!(
        "Added Project: '{}' with description '{}'",
        name, description
    );

    match models::add_project(&name, &description).await {
        Ok(_) => {
            println!("Successfully added project");
            Ok(())
        }
        Err(e) => {
            eprintln!("Failed to add project: {}", e);
            Err(format!("Failed to add project: {}", e))
        }
    }
}

#[tauri::command]
async fn update_theme(theme: &str) -> Result<(), String> {
    let theme = theme.to_string();

    println!("Updated theme to '{}'", theme);

    match models::update_theme(&theme).await {
        Ok(_) => {
            println!("Successfully updated theme");
            Ok(())
        }
        Err(e) => {
            eprintln!("Failed to update theme: {}", e);
            Err(format!("Failed to update theme: {}", e))
        }
    }
}

#[tauri::command]
async fn query_theme() -> Result<String, String> {
    match models::query_theme().await {
        Ok(theme) => {
            println!("Retrieved theme");
            Ok(theme)
        }
        Err(e) => {
            eprintln!("Failed to retrieve theme: {}", e);
            Err(format!("Failed to retrieve theme: {}", e))
        }
    }
}

#[tauri::command]
async fn list_projects() -> Result<Vec<Project>, String> {
    println!("list_projects function called");

    match models::list_projects().await {
        Ok(projects) => {
            println!("Retrieved projects");
            Ok(projects)
        }
        Err(e) => {
            eprintln!("Failed to retrieve projects: {}", e);
            Err(format!("Failed to retrieve projects: {}", e))
        }
    }
}
#[tauri::command]
async fn delete_project(project_id: i32) -> Result<(), String> {
    println!("delete_project function called");

    match models::delete_project(project_id).await {
        Ok(_) => {
            println!("Deleted project");
            Ok(())
        }
        Err(e) => {
            eprintln!("Failed to delete project: {}", e);
            Err(format!("Failed to delete project: {}", e))
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
        .invoke_handler(tauri::generate_handler![
            add_project,
            list_projects,
            update_theme,
            query_theme,
            delete_project
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
