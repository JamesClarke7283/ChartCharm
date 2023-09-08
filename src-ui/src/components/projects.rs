use crate::components::core::{AddProjectCmdArgs, DelProjectCmdArgs, EditProjectCmdArgs, Sidebar};
use crate::contexts::modal_controller::use_modal_controller;
use chartcharm_shared::Project;
use leptos::{
    component, create_action, create_resource, create_rw_signal, event_target_value,
    request_animation_frame, tracing, view, warn, For, IntoView, Resource, Scope, SignalGet,
    SignalSet,
};
use leptos_router::use_navigate;
use tauri_sys::tauri;

// Header Component
#[component]
pub fn Project_Header(cx: Scope) -> impl IntoView {
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

#[component]
pub fn Projects(cx: Scope) -> impl IntoView {
    let projects = create_resource(
        cx,
        || {},
        move |_| async move {
            let retrieved_projects = match tauri::invoke("list_projects", &()).await {
                Ok(projects) => projects,
                Err(e) => {
                    warn!("Failed to call list_projects: {}", e);
                    return Vec::<Project>::new();
                }
            };
            retrieved_projects
        },
    );

    view! {cx,
        <Project_Header />
        <ProjectList projects=projects />
    }
}

#[component]
pub fn ProjectList(cx: Scope, projects: Resource<(), Vec<Project>>) -> impl IntoView {
    view! {
        cx,
        <div class="project-list">
            <For
                each=move || {
                    let mut cloned_projects = projects.read(cx).unwrap_or_default().clone();
                    cloned_projects.sort_by(|a, b| b.updated_at.cmp(&a.updated_at));
                    cloned_projects.into_iter()
                }
                key=|project| project.id
                view=move |cx, project: Project| {
                    view!(cx, <Project_Tile project=&project />)
                }
            />
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
    let add_project = create_action(cx, move |_: &()| async move {
        tauri::invoke::<_, ()>(
            "add_project",
            &AddProjectCmdArgs {
                name: project_name.get(),
                description: project_description.get(),
            },
        )
        .await
        .unwrap_or_else(|e| {
            warn!("Failed to call add_Project: {}", e);
        });
    });
    view! { cx,
        <form id="add-project-form" on:submit=move|ev|{
            ev.prevent_default();
            add_project.dispatch(());
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

#[component]
pub fn Project_Tile<'a>(cx: Scope, project: &'a Project) -> impl IntoView {
    let modal = use_modal_controller(cx);
    let project_clone = project.clone(); // Clone the Project here
    let project_clone2 = project.clone(); // Clone the Project here

    view!(cx,
        <div class="project-tile">
        <button id="project-button" on:click=move|_| {
            let project_clone_for_closure = project_clone.clone();
            let project_id = project_clone_for_closure.id.to_string();
            request_animation_frame(move || {
                let navigate = leptos_router::use_navigate(cx);
                _ = navigate(&format!("/project/{project_id}"), Default::default());
            });
        }>
            <div class="title-container">
                <h1>{project.name.to_string()}</h1>
                <button class="icon-button" on:click=move|_| {
                    let project_clone_for_closure = project_clone2.clone();
                    modal.open(view!{cx, <Project_Options project=&project_clone_for_closure/>})
                }>
                    <i class="fa fa-ellipsis-v" aria-hidden="true"></i>
                </button>
            </div>
            <hr class="pico-divider"></hr>
            <p>{project.description.to_string()}</p>
            </button>
        </div>
    )
}

#[component]
pub fn Project_Options<'a>(cx: Scope, project: &'a Project) -> impl IntoView {
    println!("Project Options clicked");
    let modal = use_modal_controller(cx);
    let project_clone = project.clone();
    let project_clone2 = project.clone();
    view! { cx,
        <div class="project-options">
            <button class="icon-button" on:click=move|_| {
                println!("Edit pressesd");
                let project_clone_for_closure = project_clone.clone();
                modal.close();
                modal.open(view!{cx, <Project_Edit project=&project_clone_for_closure/>});
            }>
                <i class="fa fa-pencil" aria-hidden="true">Edit</i>
            </button>
            <button class="icon-button" on:click=move|_| {
                let project_clone_for_closure = project_clone2.clone();
                modal.close();
                modal.open(view!{cx, <Project_Delete_Confirmation project=&project_clone_for_closure/>})
            }>
                <i class="fa fa-trash" aria-hidden="true">Delete</i>
            </button>
        </div>
    }
}

#[component]
pub fn Project_Delete_Confirmation<'a>(cx: Scope, project: &'a Project) -> impl IntoView {
    println!("Project Delete Confirmation clicked");
    let modal = use_modal_controller(cx);
    let project_id = project.id.clone();
    let delete_project = create_action(cx, move |_: &()| async move {
        tauri::invoke::<_, ()>("delete_project", &DelProjectCmdArgs { id: project_id })
            .await
            .unwrap_or_else(|e| {
                warn!("Failed to call delete_Project: {}", e);
            });
    });
    view! { cx,
        <div id="project-delete-confirmation">
            <p>Are you sure you want to delete this project?</p>
            <button class="icon-button" on:click=move|__|{
                println!("Deleting project");
                delete_project.dispatch(());
                modal.close();
            }>
                <i class="fa fa-check" aria-hidden="true">Yes</i>
            </button>
            <button class="icon-button" on:click=move|_|modal.close()>
                <i class="fa fa-times" aria-hidden="true">No</i>
            </button>
        </div>
    }
}

#[component]
pub fn Project_Edit<'a>(cx: Scope, project: &'a Project) -> impl IntoView {
    println!("Project Edit clicked");
    let project_name = create_rw_signal(cx, String::new());
    let project_description = create_rw_signal(cx, String::new());
    let project_id = project.id.clone();

    // Set initial state for form
    project_name.set(project.name.clone());
    project_description.set(project.description.clone());

    let modal = use_modal_controller(cx);
    let edit_project = create_action(cx, move |_: &()| async move {
        tauri::invoke::<_, ()>(
            "edit_project",
            &EditProjectCmdArgs {
                id: project_id,
                name: project_name.get(),
                description: project_description.get(),
            },
        )
        .await
        .unwrap_or_else(|e| {
            warn!("Failed to call edit_project: {}", e);
        });
    });
    view! { cx,
        <form id="edit-project-form" on:submit=move|ev|{
            ev.prevent_default();
            edit_project.dispatch(());
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
