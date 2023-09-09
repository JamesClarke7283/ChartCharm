extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_attribute]
pub fn tauri_commands(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(item as DeriveInput);

    // Normally, you'd inspect `ast` to generate appropriate code
    // For this example, let's assume there is always a function called `list_projects` in src-db

    let gen = quote! {
        #[tauri::command]
        pub async fn list_projects() -> Result<Vec<Project>, String> {
            println!("list_projects function called");
            match src_db::bindings::projects::list_projects().await {
                Ok(projects) => {
                    println!("Retrieved projects");
                    Ok(projects)
                }
                Err(e) => {
                    eprintln!("Failed to retrieve projects: {}", e);
                    Err(format!("Failed to retrieve projects: {}", e))
                }
            }
        }
    };

    gen.into()
}
