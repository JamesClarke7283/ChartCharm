use futures::StreamExt;
use leptos::*;
use serde::{Deserialize, Serialize};
use tauri_sys::{event, tauri};
use leptos_meta::*;
mod components;
mod contexts;

use components::Header;
use contexts::modal_controller::*;

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





