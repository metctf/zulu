use yew::prelude::*;
use yew_router::prelude::*;
use gloo::console::log;

use crate::views::createflag::{CreateFlag, FlagData};
use crate::views::login::{Login, LoginData};
use crate::views::home::Home;
use crate::views::register::{Register, RegisterData};
use crate::views::notfound::NotFound;
use crate::views::submitflag::{FlagStringData, SubmitFlag};

#[derive(Routable, PartialEq, Eq, Clone, Debug)]
pub enum MainRoute{
    #[at("/login")]
    Login,
    #[at("/register")]
    Register,
    #[at("/")]
    Home,
    #[at("/create_flag")]
    CreateFlag,
    #[at("/submit_flag")]
    SubmitFlag,
    #[at("/settings")]
    SettingsRoot,
    #[at("/settings/*")]
    Settings,
    #[not_found]
    #[at("/404")]
    NotFound,
}


#[derive(Routable, PartialEq, Eq, Clone, Debug)]
pub enum SettingsRoute{
    #[at("/settings/modify")]
    Modify,
    #[not_found]
    #[at("/settings/404")]
    NotFound,
}

pub fn switch_main(route: MainRoute) -> Html {

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
            html! {
                <Login onsubmit={custom_form_submit} />
            }
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
            html! {
                    <Register onsubmit={custom_form_submit} />
            }
        },
        MainRoute::CreateFlag => {
            let custom_form_submit = Callback::from(|data: FlagData| {
                log!("flagid is", &data.flagid.to_string());

                wasm_bindgen_futures::spawn_local( async move {
                    let url = format!("http://127.0.0.1:8000/api/v1/create_flag");
                    let form = (
                        ("flagid",data.flagid),
                        ("challenge",data.challenge),
                        ("challengeauthor",data.challengeauthor),
                        ("flagstring", data.flagstring),
                        ("points", data.points)
                    );
                    let client = reqwest::Client::new();

                    client.post(&url)
                        .form(&form)
                        .send()
                        .await
                        .unwrap(); //Getting an error here
                });
            });
            html! {<CreateFlag onsubmit={custom_form_submit}/>}
        },
        MainRoute::SubmitFlag => {
            let custom_form_submit = Callback::from(|data: FlagStringData| {
                log!("String is", &data.flagstring);

                wasm_bindgen_futures::spawn_local( async move {
                    let url = format!("http://127.0.0.1:8000/api/v1/submit_flag/{}", &data.flagstring);
                    let client = reqwest::Client::new();

                    client.get(&url)
                        .send()
                        .await
                        .unwrap(); //Getting error here
                });
            });
            html! {
                    <SubmitFlag onsubmit={custom_form_submit} />
            }
        },
        MainRoute::Home => html! {<Home />},
        MainRoute::SettingsRoot | MainRoute::Settings => html! { <Switch<SettingsRoute> render={switch_settings} /> },
        MainRoute::NotFound => html! {<NotFound />},
    }
}

pub fn switch_settings(route: SettingsRoute) -> Html {
    match route {
        SettingsRoute::Modify => {
            let custom_form_submit = Callback::from(|data: RegisterData| {
                log!("username is", &data.username);
                log!("password is", &data.password);

                wasm_bindgen_futures::spawn_local( async move {
                    let url = format!("http://127.0.0.1:8000/api/v1/modify");
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

            let delete = Callback::from(|_| {
                wasm_bindgen_futures::spawn_local(async {
                    let url = format!("http://127.0.0.1:8000/api/v1/remove");
                    let client = reqwest::Client::new();

                    client.delete(&url)
                        .send()
                        .await
                        .unwrap();
                });
            });

            html! {
                <>
                    <Register onsubmit={custom_form_submit} />
                    <button onclick={delete}>{"Delete"}</button>
                </>
            }
        },
        SettingsRoute::NotFound => html! {<Redirect<MainRoute> to={MainRoute::NotFound}/>}
    }
}

