#![warn(
    clippy::all,
    clippy::restriction,
    clippy::pedantic,
    clippy::nursery,
)]
#![deny(warnings)]

#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

fn main() {
    match tauri::Builder::default()
        .run(tauri::generate_context!()) {
        Ok(_) => (),
        Err(e) => panic!("Could not run Tauri application: {}", e)
        }
}
