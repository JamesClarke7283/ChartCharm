#![warn(
    clippy::all,
    clippy::as_conversions,
    clippy::pedantic,
    clippy::nursery,
)]
#![allow(clippy::module_name_repetitions)]
#![deny(warnings)]

use leptos::{component, provide_context, tracing, view, warn, Scope, IntoView};
mod components;
mod contexts;

use components::Header;
use contexts::modal_controller::{ModalController, ModalViewer};

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    provide_context(cx, ModalController::new(cx));
    view! { cx,
        <ModalViewer/>
            <div>
                <Header />
            </div>
            
    }
}





