use leptos::*;
use crate::contexts::modal_controller::use_modal_controller;

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
                <button class="pico-btn pico-btn-icon" data-target="add-project-modal" id="header-add-data-button">
                    <i class="fa fa-plus" aria-hidden="true"></i>
                </button>
            </div>
        </header>
    }
}

// Sidebar Component
#[component]
pub fn Sidebar(cx: Scope) -> impl IntoView {
    view! { cx,
    <article>
        <h1>Chart Charm</h1>
        <hr class="pico-divider"></hr>
        <ul class="sidebar-menu">
        <button id="sidebar-home-btn"><li><i class="fa fa-plus"></i> Home</li></button>
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
    </article>
    }
}