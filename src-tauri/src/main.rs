#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
pub mod bindings;
use crate::bindings::charts::{
    add_chart, create_charts_table, delete_chart, list_charts, update_chart,
};
use crate::bindings::data_points::{
    add_datapoint, create_datapoints_table, delete_datapoint, list_datapoints, query_datapoint,
    update_datapoint,
};
use crate::bindings::projects::{
    add_project, create_projects_table, delete_project, edit_project, list_projects, query_project,
};
use crate::bindings::settings::{
    create_settings_table, populate_settings_table, query_settings_theme, update_settings_theme,
};

use crate::bindings::theme::{create_theme_table, populate_theme_table};

use crate::bindings::chart_kind::{create_chartkind_table, populate_chartkind_table};
use chartcharm_database::set_is_db_populated;
use tauri::Builder;
extern crate log;

use log::{debug, Level, LevelFilter, Log, Metadata, Record};
use std::env;

struct SimpleLogger;

impl Log for SimpleLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Info
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            println!("{} - {}", record.level(), record.args());
        }
    }

    fn flush(&self) {}
}

static LOGGER: SimpleLogger = SimpleLogger;

/// # Errors
/// Database errors are returned as a `DbErr` enum.
/// # Panics
/// Panics if the database connection fails.
pub fn initialize() -> Result<(), anyhow::Error> {
    debug!("Initializing database");
    // Apply all pending migrations
    create_theme_table()?;
    create_chartkind_table()?;
    create_projects_table()?;
    create_charts_table()?;
    create_datapoints_table()?;
    create_settings_table()?;
    // Initialize your tables
    debug!("Successfully created tables");
    populate_settings_table()?;
    populate_theme_table()?;
    populate_chartkind_table()?;
    set_is_db_populated(true)?;
    debug!("Successfully populated tables");
    Ok(())
}

#[async_std::main]
async fn main() {
    // Read environment variable and set log level
    let log_level = env::var("LOGLEVEL").unwrap_or_else(|_| "Info".to_string());
    let level_filter = match log_level.to_uppercase().as_str() {
        "OFF" => LevelFilter::Off,
        "ERROR" => LevelFilter::Error,
        "WARN" => LevelFilter::Warn,
        "INFO" => LevelFilter::Info,
        "DEBUG" => LevelFilter::Debug,
        "TRACE" => LevelFilter::Trace,
        _ => LevelFilter::Info, // Default is Info
    };

    log::set_logger(&LOGGER)
        .map(|()| log::set_max_level(level_filter))
        .unwrap();

    log::info!("Starting Tauri application");
    if let Err(e) = crate::initialize() {
        log::error!("Failed to initialize the database: {}", e);
    }

    Builder::default()
        .invoke_handler(tauri::generate_handler![
            add_project,
            list_projects,
            update_settings_theme,
            query_settings_theme,
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
