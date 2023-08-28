//! # My Tauri Application
//!
//! This crate provides the main entry point for the Tauri application.
//! It uses the `tauri_build::build` function to kick-start the application's build process.
//!
//! ## How to Run
//!
//! To run the application, execute `cargo run` in the terminal.
//! 
//! ## Dependencies
//! 
//! - `tauri_build` for building the application.

/// The main entry point for the Tauri application.
///
/// This function calls `tauri_build::build` to build and initialize the Tauri application.
///
/// # Examples
///
/// ```no_run
/// // Generally, this is run via `cargo run` and does not require manual execution.
/// ```
fn main() {
  tauri_build::build();
}
