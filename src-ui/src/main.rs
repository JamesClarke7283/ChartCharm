#![warn(
    clippy::all,
    clippy::as_conversions,
    clippy::pedantic,
    clippy::nursery,
)]
#![allow(clippy::module_name_repetitions)]
#![deny(warnings)]
use leptos::*;
use chartcharm_server::*;

pub fn main() {
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();
    mount_to_body(|cx| view! { cx, <App/> })
}
