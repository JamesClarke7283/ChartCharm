use crate::components::core::{AddProjectCmdArgs, UpdateThemeCmdArgs};
use crate::contexts::modal_controller::use_modal_controller;
use chartcharm_shared::Project;
use leptos::{
    component, create_action, create_resource, create_rw_signal, event_target_value, tracing, view,
    warn, For, IntoView, Resource, Scope, SignalGet, SignalSet,
};
use tauri_sys::tauri;

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

    view!(cx, <ProjectList projects=projects />)
}

#[component]
pub fn ProjectList(cx: Scope, projects: Resource<(), Vec<Project>>) -> impl IntoView {
    view! {
        cx,
        <div class="project-list">
            <For
                each=move || {
                    let cloned_projects = projects.read(cx).unwrap_or_default().clone();
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

    view!(cx,
        <div class="project-tile">
        <button id="project-button">
            <div class="title-container">
                <h1>{project.name.to_string()}</h1>
                <button class="icon-button" on:click=move|_| {
                    let project_clone_for_closure = project_clone.clone(); // Clone it again for the closure
                    modal.open(view!{cx, <ProjectOptions project=&project_clone_for_closure/>})
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
pub fn ProjectOptions<'a>(cx: Scope, project: &'a Project) -> impl IntoView {
    let modal = use_modal_controller(cx);
    view! { cx,
        <div class="project-options">
            <button class="icon-button">
                <i class="fa fa-pencil" aria-hidden="true">Edit</i>
            </button>
            <button class="icon-button">
                <i class="fa fa-trash" aria-hidden="true">Delete</i>
            </button>
        </div>
    }
}
