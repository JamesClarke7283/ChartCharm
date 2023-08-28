#![warn(
    clippy::all,
    clippy::as_conversions,
    clippy::pedantic,
    clippy::nursery,
)]
#![allow(clippy::module_name_repetitions)]
#![deny(warnings)]
use leptos::{mount_to_body, view, warn};
use chartcharm_server::App;

pub fn main() {
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();
    mount_to_body(|cx| view! { cx, <App/> });
}
