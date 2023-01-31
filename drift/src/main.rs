use yew::prelude::*;
use yew_router::prelude::*;
use gloo::console::log;

mod views;

use views::login::{Login, LoginData};
use views::home::Home;
use views::register::{Register, RegisterData};
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
        MainRoute::Login => {
            let custom_form_submit = Callback::from(|data: LoginData| {
                log!("username is", &data.username);
                log!("password is", &data.password);

                wasm_bindgen_futures::spawn_local( async move {
                    let url = format!("http://127.0.0.1:8000/api/v1/login");
                    let form = [("username",data.username), ("password", data.password)];
                    let client = reqwest::Client::new();

                    client.post(&url)
                        .form(&form)
                        .send()
                        .await
                        .unwrap(); //Getting error here
                });
            });
            html! {<Login onsubmit={custom_form_submit} />}
        },
        MainRoute::Register => {
            let custom_form_submit = Callback::from(|data: RegisterData| {
                log!("username is", &data.username);
                log!("password is", &data.password);

                wasm_bindgen_futures::spawn_local( async move {
                    let url = format!("http://127.0.0.1:8000/api/v1/register");
                    let form = [
                        ("username",data.username),
                        ("firstname",data.firstname),
                        ("lastname",data.lastname),
                        ("password", data.password),
                        ("origin", data.origin)
                    ];
                    let client = reqwest::Client::new();

                    client.post(&url)
                        .form(&form)
                        .send()
                        .await
                        .unwrap(); //Getting an error here
                });
            });
            html! {<Register onsubmit={custom_form_submit} />}
        },
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
