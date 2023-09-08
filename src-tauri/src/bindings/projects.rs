use chartcharm_database::bindings::projects;
use chartcharm_shared::Project;

#[tauri::command]
pub async fn add_project(name: &str, description: &str) -> Result<(), String> {
    let name = name.to_string();
    let description = description.to_string();

    println!(
        "Added Project: '{}' with description '{}'",
        name, description
    );

    match projects::add_project(&name, &description).await {
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
pub async fn list_projects() -> Result<Vec<Project>, String> {
    println!("list_projects function called");

    match projects::list_projects().await {
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
pub async fn delete_project(id: u16) -> Result<(), String> {
    println!("delete_project function called");

    match projects::delete_project(id).await {
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

#[tauri::command]
pub async fn edit_project(id: u16, name: &str, description: &str) -> Result<(), String> {
    println!("edit_project function called");
    match projects::edit_project(id, name, description).await {
        Ok(_) => {
            println!("Edited project");
            Ok(())
        }
        Err(e) => {
            eprintln!("Failed to edit project: {}", e);
            Err(format!("Failed to edit project: {}", e))
        }
    }
}
