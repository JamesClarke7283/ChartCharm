use chartcharm_server::models::*;
use chartcharm_server::App;
use leptos::{mount_to_body, view};
use sea_orm::Database;

#[async_std::main]
pub async fn main() {
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();
    mount_to_body(|cx| view! { cx, <App/> });

    // Connect to database
    let db = match Database::connect(&db_string()).await {
        Ok(db) => db,
        Err(e) => {
            panic!("Failed to connect to database: {:?}", e);
        }
    };

    // Populate database with default data
    if check_empty(&db).await {
        populate(&db).await;
    }
}
