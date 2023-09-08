use leptos::{
    component, create_rw_signal, tracing, use_context, view, warn, IntoView, ReadSignal, RwSignal,
    SignalGet, SignalSet, View,
};
/// # Modal Controller
///
/// The `ModalController` struct is responsible for managing modals within the application.
///
/// ## Fields
///
/// - `modal: RwSignal<Option<View>>` - Read-write signal to manage the current modal view.
/// - `current_modal: ReadSignal<Option<View>>` - Read-only signal representing the current modal view.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ModalController {
    pub modal: RwSignal<Option<View>>,
    pub current_modal: ReadSignal<Option<View>>,
}

impl ModalController {
    /// Create a new `ModalController`.
    ///
    /// Initializes a new `ModalController` with a `None` modal.
    ///
    /// ## Parameters
    ///
    /// - `` - The scope of the component.
    ///
    /// ## Returns
    ///
    /// Returns a `ModalController` instance.
    pub fn new() -> Self {
        let modal = create_rw_signal(None);
        Self {
            modal,
            current_modal: modal.read_only(),
        }
    }

    /// Open a new modal.
    ///
    /// Replaces the current modal with a new one.
    ///
    /// ## Parameters
    ///
    /// - `modal: View` - The new modal view to be opened.
    pub fn open(&self, modal: View) {
        self.modal.set(Some(modal));
    }

    /// Close the current modal.
    ///
    /// Sets the current modal to `None`, effectively closing it.
    pub fn close(&self) {
        self.modal.set(None);
    }
}

/// # Modal Viewer Component
///
/// A component for displaying the current modal, if any exists.
///
/// ## Parameters
///
/// - `` - The scope of the component.
///
/// ## Returns
///
/// An implementation of `IntoView` for rendering the modal.
#[component]
pub fn ModalViewer() -> impl IntoView {
    let modal_controller = use_modal_controller();
    view! {
        <dialog open=move||modal_controller.current_modal.get().is_some() on:click=move|_|modal_controller.close()>
            <article on:click=|ev|ev.stop_propagation()>
                {move || {
                    if let Some(modal) = modal_controller.current_modal.get() {
                        #[allow(unused_braces)]
                        return Some(view! { {modal}});
                    }
                    None
                }}
            </article>
        </dialog>
    }
}

/// Retrieve the current `ModalController` context.
///
/// ## Parameters
///
/// - `` - The scope of the component.
///
/// ## Returns
///
/// Returns a `ModalController` instance.
///
/// ## Panics
///
/// Panics if unable to get the current modal context.
pub fn use_modal_controller() -> ModalController {
    use_context().expect("unable to get current modal context")
}
