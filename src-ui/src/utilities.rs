use wasm_bindgen::JsCast;
use web_sys::window;


pub fn set_displayed_theme(theme: &str) {
    println!("Setting displayed theme to '{}'", theme);
    let window = window().expect("should have a window in this context");
    let document = window.document().expect("should have a document on window");
    let document_element = document.document_element().expect("document should have a root element");

    document_element
        .dyn_into::<web_sys::Element>()
        .expect("document element should be an Element")
        .set_attribute("data-theme", theme)
        .expect("Failed to set attribute");
}
