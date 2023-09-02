use leptos::{component, provide_context, tracing, view, warn, IntoView, Scope, create_action};
pub mod components;
pub mod contexts;
pub mod utilities;
use components::{Header, ProjectList};
use contexts::modal_controller::{ModalController, ModalViewer};
use tauri_sys::tauri;
use crate::utilities::set_displayed_theme;

/// # App Component
///
/// The main `App` component for the application's front-end.
///
/// This function initializes the main view of the application,
/// which includes other components like `ModalViewer` and `Header`.
/// It also sets up the context for modals.
///
/// ## Parameters
///
/// - `cx: Scope` - The scope of the component.
///
/// ## Returns
///
/// An implementation of `IntoView`, which is the main view for the application.
///
/// ## Examples
///
/// ```no_run
/// // Initialize the App component
/// App(cx);
/// ```
///
/// ## Dependencies
///
/// - `ModalController` for managing modals.
/// - `ModalViewer` for viewing modals.
/// - `Header` for the header component.
#[component]
pub fn App(cx: Scope) -> impl IntoView {
    // Create action to query theme
    let init_theme = create_action(cx, move |_| async move {
        match tauri::invoke::<(), String>("query_theme", &()).await {
            Ok(retrieved_theme) => {
                set_displayed_theme(&retrieved_theme);  // Replace with your actual function to set theme
            },
            Err(e) => {
                warn!("Failed to call query_theme: {}", e);
            }
        }
    });
    
    // Dispatch the action to query and set theme at startup
    init_theme.dispatch(&());

    provide_context(cx, ModalController::new(cx));
    view! { cx,
        <ModalViewer/>
            <div>
                <Header />
                <ProjectList />
            </div>
    }
}
