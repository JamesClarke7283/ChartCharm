use chartcharm_database::bindings::settings;

#[tauri::command]
pub async fn update_theme(theme: &str) -> Result<(), String> {
    let theme = theme.to_string();

    println!("Updated theme to '{}'", theme);

    match settings::update_theme(&theme).await {
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
pub async fn query_theme() -> Result<String, String> {
    match settings::query_theme().await {
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
