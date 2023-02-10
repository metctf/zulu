use gloo_storage::errors::StorageError;
use yew::prelude::*;
use yew_router::prelude::*;
use gloo::console::log;
use gloo::storage::LocalStorage;
use gloo_storage::Storage;

use crate::views::components::top_bar::{NavBar, Tab};
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
                    let client = reqwest::Client::builder()
                        .build()
                        .unwrap();

                    let cookie = client.post(&url)
                        .form(&form)
                        .send()
                        .await
                        .unwrap()
                        .text()
                        .await
                        .unwrap();

                        LocalStorage::set("Response", &cookie).unwrap();
                        
                });
            });
            let string: Result<String,StorageError>= LocalStorage::get("Response"); 
            let auth: Tab;

            match string {
                Ok(string) => {
                    if string.eq("Successfully authenticated!") {
                        auth = Tab::Authorized;
                    } else {
                        auth = Tab::Unauthorized;
                    }
                },
                Err(_) => {
                    auth = Tab::Unauthorized;
                }
            }
            html! {
                <>
                    <NavBar tab={auth}/>
                    <Login onsubmit={custom_form_submit} />
                </>
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
                <>
                    <NavBar tab={Tab::Authorized}/>
                    <Register onsubmit={custom_form_submit} />
                </>
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
            html! {
                <>
                    <NavBar tab={Tab::Settings}/>
                    <CreateFlag onsubmit={custom_form_submit}/>
                </>
            }
        },
        MainRoute::SubmitFlag => {
            let custom_form_submit = Callback::from(|data: FlagStringData| {
                log!("String is", &data.flagstring);

                wasm_bindgen_futures::spawn_local( async move {
                    let url = format!("http://127.0.0.1:8000/api/v1/submit_flag/{}", &data.flagstring);
                    reqwest::Client::new()
                        .get(&url)
                        .send()
                        .await
                        .unwrap();

                });
            });
            html! {
                <>
                    <NavBar tab={Tab::Authorized}/>
                    <SubmitFlag onsubmit={custom_form_submit} />
                </>
            }
        },
        MainRoute::Home => html! {
            <>
                <NavBar tab={Tab::Unauthorized}/>
                <Home />
        </>
        },
        MainRoute::SettingsRoot | MainRoute::Settings => html! { <Switch<SettingsRoute> render={switch_settings} /> },
        MainRoute::NotFound => html! {
            <>
                <NavBar tab={Tab::Unauthorized}/>
                <NotFound />
        </>
        },
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
                    <NavBar tab={Tab::Unauthorized}/>
                    <Register onsubmit={custom_form_submit} />
                    <button onclick={delete}>{"Delete"}</button>
                </>
            }
        },
        SettingsRoute::NotFound => html! {<Redirect<MainRoute> to={MainRoute::NotFound}/>}
    }
}

