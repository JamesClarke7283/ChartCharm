#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use tauri::Builder;

fn main() {
    Builder::default()
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
