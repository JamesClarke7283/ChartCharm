#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use chartcharm_database::init_db;
use tauri::Builder;

#[async_std::main]
async fn main() {
    println!("Starting Tauri application");
    init_db().await;

    Builder::default()
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
