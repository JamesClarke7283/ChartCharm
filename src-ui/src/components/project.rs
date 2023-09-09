use std::process::id;

use crate::components::core::{QueryProjectCmdArgs, Sidebar};
use crate::contexts::modal_controller::use_modal_controller;
use chartcharm_shared::{Project, ProjectError};
use chrono::Utc;
use leptos::{
    component, create_effect, create_resource, create_rw_signal, event_target_value, spawn_local,
    tracing, view, warn, IntoView, Params, SignalGet, SignalSet, SignalWith, Suspense,
};
use leptos_router::*;
use log::info;
use tauri_sys::tauri;

#[component]
pub fn ProjectHeader<'poo>(project: &'poo chartcharm_shared::Project) -> impl IntoView {
    let modal = use_modal_controller();
    let project = project.clone();
    view! {
        <header id="header" class="pico-container pico-bg-primary foreground-widget">
        // Burger Menu Icon
        <div id="header-burger-menu-container" class="transparent-action">
            <button class="pico-btn pico-btn-icon" id="header-burger-menu-button" on:click=move|_|modal.open(view!{ <Sidebar/>})>
                <i class="fa fa-bars" aria-hidden="true"></i>
            </button>
        </div>

        // App Name: Chart Charm
        <div id="header-text-container">
            <h1 class="pico-h3 pico-mb-0">{project.name}</h1>
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
            /*
            <button class="pico-btn pico-btn-icon" id="header-add-data-button" on:click=move|_|modal.open(view!{ <Add_Project/>})>
                <i class="fa fa-plus" aria-hidden="true"></i>
            </button>
            */
        </div>
    </header>

    }
}

#[component]
pub fn Project() -> impl IntoView {
    let params = use_params_map();
    let modal = use_modal_controller();
    let id = move || params.with(|params| params.get("id").cloned());

    let id_str = id().unwrap_or_else(|| "0".to_string());
    let id = id_str.parse::<u16>().unwrap_or(0); // replace unwrap with proper error handling

    let project = create_rw_signal(chartcharm_shared::Project {
        id: 0,
        name: String::new(),
        description: String::new(),
        created_at: Utc::now(),
        updated_at: Utc::now(),
    });

    let query_project = create_resource(
        || {},
        move |_| async move {
            let retrieved_project =
                match tauri::invoke("query_project", &QueryProjectCmdArgs { id: id }).await {
                    Ok(projects) => {
                        info!("project: {projects:?}");
                        projects
                    }
                    Err(e) => {
                        warn!("Failed to call list_projects: {}", e);
                        return chartcharm_shared::Project {
                            id: 0,
                            name: String::new(),
                            description: String::new(),
                            created_at: Utc::now(),
                            updated_at: Utc::now(),
                        };
                    }
                };
            retrieved_project
        },
    );

    let project = move || match query_project.get() {
        Some(project) => project.clone(),
        None => {
            warn!("Failed to get project with id '{id}'");
            chartcharm_shared::Project {
                id: 0,
                name: String::new(),
                description: String::new(),
                created_at: Utc::now(),
                updated_at: Utc::now(),
            }
        }
    };
    view! {
        <Suspense fallback=|| view!{<p>{"Loading..."}</p>}>
        {move ||{
            query_project.get().map(|project| {
                view! {
                    <ProjectHeader project=&project/>
                }
            })
        }
    }
    </Suspense>
    }
}
