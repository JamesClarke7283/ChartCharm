use leptos::{component, create_action, provide_context, tracing, view, warn, IntoView};
use leptos_router::{Route, Router, Routes};
pub mod components;
pub mod contexts;
pub mod utilities;
use crate::components::project::Project;
use crate::utilities::set_displayed_theme;
use components::projects::Projects;
use contexts::modal_controller::{ModalController, ModalViewer};
use tauri_sys::tauri;

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
/// - `` - The scope of the component.
///
/// ## Returns
///
/// An implementation of `IntoView`, which is the main view for the application.
///
/// ## Examples
///
/// ```no_run
/// // Initialize the App component
/// App();
/// ```
///
/// ## Dependencies
///
/// - `ModalController` for managing modals.
/// - `ModalViewer` for viewing modals.
/// - `Header` for the header component.
#[component]
pub fn App() -> impl IntoView {
    // Create action to query theme
    let init_theme = create_action(move |_| async move {
        match tauri::invoke::<(), String>("query_theme", &()).await {
            Ok(retrieved_theme) => {
                set_displayed_theme(&retrieved_theme); // Replace with your actual function to set theme
            }
            Err(e) => {
                warn!("Failed to call query_theme: {}", e);
            }
        }
    });

    // Dispatch the action to query and set theme at startup
    init_theme.dispatch(&());

    provide_context(ModalController::new());
    view! {
        <Router>
        <ModalViewer/>
        <main>
            <div id="route-view">
            <Routes>
            <Route path="/" view=Projects/>
            <Route path="/project/:id" view=Project/>
            </Routes>
            </div>
            </main>
        </Router>
    }
}
