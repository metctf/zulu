use yew::prelude::*;
use yew_router::prelude::*;

mod router;
mod pages;
mod components;
mod forms;
mod settings;

use crate::router::{MainRoute,switch_main};


#[function_component]
fn App() -> Html {

    html! {
        <BrowserRouter>
            <Switch<MainRoute> render={switch_main} />
        </BrowserRouter>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
