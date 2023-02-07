use yew::prelude::*;
use yew_router::prelude::*;

mod views;
mod router;

use views::components::top_bar::{Tab, NavBar};
use crate::router::{MainRoute,switch_main};


#[function_component]
fn App() -> Html {

    html! {
        <BrowserRouter>
            <NavBar tab={Tab::Unauthorized}/>
            <Switch<MainRoute> render={switch_main} />
        </BrowserRouter>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
