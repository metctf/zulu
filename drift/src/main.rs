use yew::prelude::*;
use yew_router::prelude::*;

mod views;

use views::login::Login;
use views::home::Home;
use views::register::Register;
use views::notfound::NotFound;
use views::settings::delete::Delete;
use views::settings::modify::Modify;

#[derive(Routable, PartialEq, Eq, Clone, Debug)]
enum MainRoute{
    #[at("/login")]
    Login,
    #[at("/register")]
    Register,
    #[at("/")]
    Home,
    #[at("/settings")]
    SettingsRoot,
    #[at("/settings/*")]
    Settings,
    #[not_found]
    #[at("/404")]
    NotFound,
}


#[derive(Routable, PartialEq, Eq, Clone, Debug)]
enum SettingsRoute{
    #[at("/settings/modify")]
    Modify,
    #[at("/settings/delete")]
    Delete,
    #[not_found]
    #[at("/settings/404")]
    NotFound,
}

fn switch_main(route: MainRoute) -> Html {
    match route {
        MainRoute::Login => html! {<Login />},
        MainRoute::Register => html! {<Register />},
        MainRoute::Home => html! {<Home />},
        MainRoute::SettingsRoot | MainRoute::Settings => html! { <Switch<SettingsRoute> render={switch_settings} /> },
        MainRoute::NotFound => html! {<NotFound />},
    }
}

fn switch_settings(route: SettingsRoute) -> Html {
    match route {
        SettingsRoute::Modify => html! {<Modify />},
        SettingsRoute::Delete => html! {<Delete />},
        SettingsRoute::NotFound => html! {<Redirect<MainRoute> to={MainRoute::NotFound}/>}
    }
}


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
