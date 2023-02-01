use yew::prelude::*;
use yew_router::prelude::*;

mod views;
mod router;

use views::components::top_bar::TopBarComponent;
use crate::router::{MainRoute,switch_main};


#[function_component]
fn App() -> Html {

    html! {
        <BrowserRouter>
            <TopBarComponent />
            <Switch<MainRoute> render={switch_main} />
        </BrowserRouter>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
