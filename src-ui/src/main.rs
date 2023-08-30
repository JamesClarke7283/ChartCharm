use chartcharm_database::init_db;
use chartcharm_server::App;
use leptos::{mount_to_body, view};

#[async_std::main]
pub async fn main() {
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();
    mount_to_body(|cx| view! { cx, <App/> });

    match init_db().await {
        true => {}
        false => {
            panic!("Failed to connect to database");
        }
    }
}
