#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use chartcharm_database::init_db;
use tauri::Builder;

#[async_std::main]
async fn main() {
    Builder::default()
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
    match init_db().await {
        true => {}
        false => {
            panic!("Failed to connect to database");
        }
    };
}
