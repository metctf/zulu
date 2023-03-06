use gloo_storage::LocalStorage;
use gloo::storage::Storage;
use gloo::console::log;
use gloo::utils::document;
use yew::prelude::*;
use yew_router::prelude::*;

mod router;
mod pages;
mod components;
mod forms;
mod settings;

use crate::router::{MainRoute,switch_main};

pub fn theme_selector(){
    let theme: Result<String,_> = LocalStorage::get("theme");
    let root_element = document().document_element().unwrap();

    match theme {
        Ok(theme) => {
            match theme.as_str() {
                "light" => {
                    root_element.set_attribute("theme", "light").unwrap();
                },
                "dark" => {
                    root_element.set_attribute("theme", "dark").unwrap();
                },
                _ => {
                    log!(format!("Theme not found"));
                }
            }
        },
        Err(_) => {
            root_element.set_attribute("theme", "light").unwrap();
        }
    }
}

#[function_component]
fn App() -> Html {

    theme_selector();

    html! {
        <BrowserRouter>
            <Switch<MainRoute> render={switch_main} />
        </BrowserRouter>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
