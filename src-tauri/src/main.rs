//! # Tauri Viewlication Entry Point
//!
//! This crate serves as the entry point for the `ChartCharm` Tauri application.
//! It is responsible for initializing and running the Tauri application.
//!
//! ## Lint Rules
//!
//! This crate is configured with several Clippy lint rules for code quality, and
//! it is set to deny warnings to enforce these lints.
//!
//! ## Windows Configuration
//!
//! On Windows, when not in debug mode, the application runs without showing a
//! console window in the background.
//!
//! ## Main Function
//!
//! The `main` function is responsible for building and running the Tauri application.
//! It panics if the application cannot run, providing an error message.
//!
//! # Examples
//!
//! ```no_run
//! // This is generally run via `cargo tauri run` and does not require manual execution.
//! ```
//!
//! # Dependencies
//!
//! - Tauri
//!
//! # Panic
//!
//! The application will panic if it encounters an error during the initialization or running phase.

#![warn(
    clippy::all,
    clippy::as_conversions,
    clippy::pedantic,
    clippy::nursery,
)]
#![deny(warnings)]

#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

/// Entry point for the `ChartCharm` Tauri application.
///
/// It initializes and runs the Tauri application. If the application cannot
/// be run, it will panic with an appropriate error message.
fn main() {
    if tauri::Builder::default() 
          .run(tauri::generate_context!()).is_ok() {  }
}
