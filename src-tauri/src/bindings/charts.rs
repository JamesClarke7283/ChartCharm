use chartcharm_database::bindings::charts;
use chartcharm_shared::Chart;

#[tauri::command]
pub async fn list_charts() -> Result<Vec<Chart>, String> {
    println!("list_charts function called");

    match charts::list_charts().await {
        Ok(charts) => {
            println!("Retrieved charts");
            Ok(charts)
        }
        Err(e) => {
            eprintln!("Failed to retrieve charts: {}", e);
            Err(format!("Failed to retrieve charts: {}", e))
        }
    }
}

#[tauri::command]
pub async fn query_chart(id: u16) -> Result<Chart, String> {
    println!("query_chart function called");
    match charts::query_chart(id).await {
        Ok(chart) => {
            println!("Retrieved chart");
            Ok(chart)
        }
        Err(e) => {
            eprintln!("Failed to retrieve chart: {}", e);
            Err(format!("Failed to retrieve chart: {}", e))
        }
    }
}

#[tauri::command]
pub async fn add_chart(
    name: &str,
    description: &str,
    project: u16,
    kind: u8,
) -> Result<(), String> {
    let name = name.to_string();
    let description = description.to_string();
    let project = project;
    let kind = kind;

    println!("Added Chart: '{}' with description '{}'", name, description);

    match charts::add_chart(name.to_string(), description.to_string(), project, kind).await {
        Ok(_) => {
            println!("Successfully added chart");
            Ok(())
        }
        Err(e) => {
            eprintln!("Failed to add chart: {}", e);
            Err(format!("Failed to add chart: {}", e))
        }
    }
}

#[tauri::command]
pub async fn delete_chart(id: u16) -> Result<(), String> {
    println!("delete_chart function called");

    match charts::delete_chart(id).await {
        Ok(_) => {
            println!("Deleted chart");
            Ok(())
        }
        Err(e) => {
            eprintln!("Failed to delete chart: {}", e);
            Err(format!("Failed to delete chart: {}", e))
        }
    }
}

#[tauri::command]
pub async fn update_chart(
    id: u16,
    name: &str,
    description: &str,
    project: u16,
    kind: u8,
) -> Result<(), String> {
    println!("update_chart function called");
    match charts::update_chart(id, name.to_string(), description.to_string(), project, kind).await {
        Ok(_) => {
            println!("Updated chart");
            Ok(())
        }
        Err(e) => {
            eprintln!("Failed to update chart: {}", e);
            Err(format!("Failed to update chart: {}", e))
        }
    }
}
