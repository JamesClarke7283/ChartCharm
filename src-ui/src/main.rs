//! # `ChartCharm` Rust
//!
//! `chartcharm_rust` is a Rust crate that serves as the front-end for the `ChartCharm` application.
//! It uses the `leptos` web framework for rendering the UI and `chartcharm_server` for the back-end logic.
//!
//! This crate is configured to adhere to strict linting rules via Clippy.
//!
//! ## Features
//!
//! - Web UI using the `leptos` framework
//! - Strict linting with Clippy
//! - Panic hook for better debugging
//!
//! ## Examples
//!
//! ```no_run
//! // Run the application
//! chartcharm_rust::main();
//! ```
//!
//! ## Dependencies
//!
//! - `leptos`
//! - `chartcharm_server`
//! - `console_log`
//! - `console_error_panic_hook`

#![warn(
    clippy::all,
    clippy::as_conversions,
    clippy::pedantic,
    clippy::nursery,
)]
#![deny(warnings)]
#![feature(stmt_expr_attributes)]
use leptos::{mount_to_body, view, warn};
use chartcharm_server::components::App;

/// The main entry point for the front-end application.
///
/// It initializes the logging and panic hook, and mounts the `App` component
/// from `chartcharm_server` onto the HTML body.
///
/// # Panics
///
/// Panics if the `console_error_panic_hook` fails to set.
///
/// # Examples
///
/// ```no_run
/// // Run the application
/// chartcharm_rust::main();
/// ```
pub fn main() {
    #[must_use]
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();
    mount_to_body(|cx| view! { cx, <App/> });
}
