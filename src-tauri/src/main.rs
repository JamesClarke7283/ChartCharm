#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
pub mod bindings;
use crate::bindings::charts::{add_chart, delete_chart, list_charts, update_chart};
use crate::bindings::data_points::{
    add_datapoint, delete_datapoint, list_datapoints, query_datapoint, update_datapoint,
};
use crate::bindings::projects::{
    add_project, delete_project, edit_project, list_projects, query_project,
};
use crate::bindings::settings::{query_theme, update_theme};
use tauri::Builder;

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
            delete_project,
            edit_project,
            query_project,
            add_chart,
            delete_chart,
            list_charts,
            update_chart,
            add_datapoint,
            delete_datapoint,
            list_datapoints,
            update_datapoint,
            query_datapoint
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
