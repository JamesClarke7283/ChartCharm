use futures::StreamExt;
use leptos::*;
use serde::{Deserialize, Serialize};
use tauri_sys::{event, tauri};
use leptos_meta::*;
mod components;
mod contexts;

use components::header::Header;
use components::sidebar::Sidebar;
use contexts::modal_controller::*;

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    /*provide_meta_context(cx);*/
    provide_context(cx, ModalController::new(cx));
    view! { cx,
        <ModalViewer/>
            <div>
                <Header />
            </div>
            
    }
}





