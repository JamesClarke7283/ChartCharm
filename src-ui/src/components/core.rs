use crate::components::projects::Add_Project;
use crate::contexts::modal_controller::use_modal_controller;
use crate::utilities::set_displayed_theme;
use chartcharm_shared::Project;
use leptos::{
    component, create_action, create_resource, create_rw_signal, event_target_value, tracing, view,
    warn, For, IntoView, Scope, SignalGet, SignalSet,
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
pub struct DelProjectCmdArgs {
    pub projectId: i32,
}

#[derive(Serialize)]
pub struct UpdateThemeCmdArgs {
    pub theme: String,
}

// Header Component
#[component]
pub fn Header(cx: Scope) -> impl IntoView {
    let modal = use_modal_controller(cx);
    view! { cx,
        <header id="header" class="pico-container pico-bg-primary foreground-widget">
            // Burger Menu Icon
            <div id="header-burger-menu-container" class="transparent-action">
                <button class="pico-btn pico-btn-icon" id="header-burger-menu-button" on:click=move|_|modal.open(view!{cx, <Sidebar/>})>
                    <i class="fa fa-bars" aria-hidden="true"></i>
                </button>
            </div>

            // App Name: Chart Charm
            <div id="header-text-container">
                <h1 class="pico-h3 pico-mb-0">"Chart Charm"</h1>
            </div>

            // Icons: Import, Export, and Plus
            <div id="header-actions-container" class="transparent-action">
                // Import Icon
                <button class="pico-btn pico-btn-icon" id="header-import-data-button">
                    <i class="fa fa-upload" aria-hidden="true"></i>
                </button>

                // Export Icon
                <button class="pico-btn pico-btn-icon" id="header-export-data-button">
                    <i class="fa fa-download" aria-hidden="true"></i>
                </button>

                // Plus Icon
                <button class="pico-btn pico-btn-icon" id="header-add-data-button" on:click=move|_|modal.open(view!{cx, <Add_Project/>})>
                    <i class="fa fa-plus" aria-hidden="true"></i>
                </button>
            </div>
        </header>
    }
}

// Sidebar Component
#[component]
pub fn Sidebar(cx: Scope) -> impl IntoView {
    let modal = use_modal_controller(cx);
    let theme = create_rw_signal(cx, String::new());

    let query_theme = create_action(cx, move |_| async move {
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

    let update_theme = create_action(cx, move |_| async move {
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

    view! { cx,
        <h1>Chart Charm</h1>
        <hr class="pico-divider"></hr>
        <ul class="sidebar-menu">
        <button id="sidebar-home-btn" on:click=move|_|modal.close()><li><i class="fa fa-plus"></i> Home</li></button>
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
