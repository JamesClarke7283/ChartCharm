use crate::contexts::modal_controller::use_modal_controller;
use crate::utilities::set_displayed_theme;
use leptos::{
    component, create_action, create_rw_signal, event_target_value, request_animation_frame,
    tracing, view, warn, IntoView, SignalGet, SignalSet,
};
use serde::Serialize;
use tauri_sys::tauri;
use wasm_bindgen::JsCast;
use web_sys::HtmlSelectElement;

#[derive(Serialize)]
pub struct AddProjectCmdArgs {
    pub name: String,
    pub description: String,
}

#[derive(Serialize)]
pub struct EditProjectCmdArgs {
    pub id: u16,
    pub name: String,
    pub description: String,
}

#[derive(Serialize)]
pub struct DelProjectCmdArgs {
    pub id: u16,
}

#[derive(Serialize)]
pub struct UpdateThemeCmdArgs {
    pub theme: String,
}

#[derive(Serialize)]
pub struct QueryProjectCmdArgs {
    pub id: u16,
}

#[derive(Serialize)]
pub struct ListDataPointsCmdArgs {
    pub project: u16,
}

#[derive(Serialize)]
pub struct AddDataPointCmdArgs {
    pub project: u16,
    pub data: f32,
}

// Sidebar Component
#[component]
pub fn Sidebar() -> impl IntoView {
    let modal = use_modal_controller();
    let theme = create_rw_signal(String::new());

    let query_theme = create_action(move |_| async move {
        match tauri::invoke::<(), String>("query_theme", &()).await {
            Ok(retrieved_theme) => {
                theme.set(retrieved_theme.clone());
                set_displayed_theme(&theme.get());

                // New code to set the initial value of the select box
                let window = web_sys::window().expect("should have a window in this context");
                let document = window.document().expect("should have a document on window");
                if let Some(select_box) = document.get_element_by_id("theme-switcher") {
                    let select_box: HtmlSelectElement = select_box.dyn_into().unwrap();
                    select_box.set_value(&retrieved_theme);
                }
            }
            Err(e) => {
                warn!("Failed to call query_theme: {}", e);
            }
        }
    });

    query_theme.dispatch(&());

    let update_theme = create_action(move |_| async move {
        match tauri::invoke::<UpdateThemeCmdArgs, ()>(
            "update_theme",
            &UpdateThemeCmdArgs { theme: theme.get() },
        )
        .await
        {
            Ok(_) => {
                set_displayed_theme(&theme.get());
            }
            Err(e) => {
                warn!("Failed to call set_theme: {}", e);
            }
        }
    });

    view! {
        <h1>Chart Charm</h1>
        <hr class="pico-divider"></hr>
        <ul class="sidebar-menu">
        <button id="sidebar-home-btn" on:click=move|_|{
            modal.close();
            request_animation_frame(move || {
                let navigate = leptos_router::use_navigate();
                _ = navigate("/", Default::default());
            });
        }><li><i class="fa fa-plus"></i> Home</li></button>
        <li><i class="fa fa-bell"></i> Reminders</li>
        <li><i class="fa fa-pencil"></i> Notes</li>
        <li><i class="fa fa-undo"></i> Backup and Restore</li>
        </ul>
        <hr class="pico-divider"></hr>
        <ul class="sidebar-menu">
        <li><i class="fa fa-question-circle"></i> FAQ</li>
        <li><i class="fa fa-star"></i> Rate the app</li>
        <li><i class="fa fa-info-circle"></i> About</li>
        </ul>
        <hr class="pico-divider"></hr>
        <div>
        <i class="fa fa-paint-brush"></i>
        Theme
        <select id="theme-switcher" on:change=move|ev|{
            println!("Theme changed to: {}", event_target_value(&ev));
            theme.set(event_target_value(&ev));
            update_theme.dispatch(&());
        } prop:value=theme.get()>
            <option value="auto">OS Default</option>
            <option value="light">Light</option>
            <option value="dark">Dark</option>
        </select>
        </div>
    }
}
