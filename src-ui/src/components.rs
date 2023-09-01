use crate::contexts::modal_controller::use_modal_controller;
use leptos::{
    component, create_rw_signal, event_target_value, tracing, view, warn, IntoView, Scope,
    SignalGet, SignalSet,
};
use serde::Serialize;
use tauri_sys::tauri;

#[derive(Serialize)]
struct AddProjectCmdArgs {
    name: String,
    description: String,
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
        <select id="theme-switcher">
            <option value="auto">OS Default</option>
            <option value="light">Light</option>
            <option value="dark">Dark</option>
        </select>
        </div>
    }
}

/// # Add Project Component
///
/// A component for adding a new project. This component includes a form
/// with fields for the project's name and description, as well as "Save"
/// and "Cancel" buttons.
///
/// ## Parameters
///
/// - `cx: Scope` - The scope of the component.
#[component]
pub fn Add_Project(cx: Scope) -> impl IntoView {
    let modal = use_modal_controller(cx);
    let project_name = create_rw_signal(cx, String::new());
    let project_description = create_rw_signal(cx, String::new());
    view! { cx,
        <form id="add-project-form" on:submit=move|ev|{
            ev.prevent_default();
            async_std::task::spawn_local(async move {
                let _ = tauri::invoke::<_, ()>("add_project", &AddProjectCmdArgs {
                    name: project_name.get(),
                    description: project_description.get(),
                })
                .await
                .unwrap_or_else(|e| {
                    warn!("Failed to call add_Project: {}", e);
                });
            });


            //add_project(project_name.get(), project_description.get());
            modal.close();
        }>
            <label for="project-name">Project Name:</label>
            <input type="text" id="project-name" name="project-name" on:input=move|ev|project_name.set(event_target_value(&ev)) prop:value=move||project_name.get() required />

            <label for="project-description">Project Description:</label>
            <textarea id="project-description" name="project-description" on:input=move|ev|project_description.set(event_target_value(&ev)) prop:value=move||project_description.get()></textarea>

            <button type="submit">Save</button>
            <button type="button" on:click=move|_|modal.close() {
            }>Cancel</button>
        </form>
    }
}
