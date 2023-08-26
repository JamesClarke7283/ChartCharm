//! # `ChartCharm` Library
//!
//! `chartcharm_lib` is a Rust library that serves as the core logic for the `ChartCharm` front-end application.
//! It leverages the `leptos` web framework for creating UI components and managing contexts.
//!
//! This library is strictly linted using various Clippy rules to ensure high code quality.
//!
//! ## Features
//!
//! - UI components like `Header` and `Sidebar` for the web application.
//! - Context management for modals via `ModalController`.
//! - Strict linting through Clippy.
//!
//! ## Usage
//!
//! ```no_run
//! // Initialize the App component
//! chartcharm_lib::App(cx);
//! ```
//!
//! ## Dependencies
//!
//! - `leptos`
//! - `components`
//! - `contexts`

#![warn(
    clippy::all,
    clippy::as_conversions,
    clippy::pedantic,
    clippy::nursery,
)]
#![deny(warnings)]

use leptos::{component, provide_context, tracing, view, warn, Scope, IntoView};
mod components;
mod contexts;

use components::Header;
use contexts::modal_controller::{ModalController, Viewer};

/// The main `App` component for the ChartCharm front-end.
///
/// This function initializes the main view of the application, which includes
/// components like `Header` and `Sidebar`, and also sets up context management for modals.
///
/// # Parameters
///
/// - `cx: Scope` - The scope of the component.
///
/// # Returns
///
/// An implementation of `IntoView` which is the main view for the application.
///
/// # Examples
///
/// ```no_run
/// // Initialize the App component
/// chartcharm_lib::App(cx);
/// ```
#[component]
pub fn App(cx: Scope) -> impl IntoView {
    provide_context(cx, ModalController::new(cx));
    view! { cx,
        <Viewer/>
            <div>
                <Header />
            </div>
            
    };
}
