use yew::prelude::*;
use std::ops::Deref;
use gloo_storage::errors::StorageError;
use yew_router::prelude::*;
use gloo::console::log;
use gloo::storage::LocalStorage;
use gloo_storage::Storage;

use crate::views::components::top_bar::{NavBar, Tab};
use crate::MainRoute;

use super::components::text_input::TextInput;
use super::components::custom_button::CustomButton;

#[derive(Default, Clone)]
pub struct LoginData {
    pub username: String,
    pub password: String,
}

#[derive(Properties, PartialEq)]
pub struct Props {
    pub onsubmit: Callback<LoginData>,
}


#[function_component(Login)]
pub fn login(props: &Props) -> Html {

    let state = use_state(|| LoginData::default());

    let cloned_state = state.clone();
    let username_changed = Callback::from(move |username| {
        let mut data = cloned_state.deref().clone();
        data.username = username;
        cloned_state.set(data);
    });

    let cloned_state = state.clone();
    let password_changed = Callback::from(move |language| {
        let mut data = cloned_state.deref().clone();
        data.password = language;
        cloned_state.set(data);
    });

    let form_onsubmit = props.onsubmit.clone();
    let cloned_state = state.clone();
    let onsubmit = Callback::from(move |event: SubmitEvent| {
        event.prevent_default();
        let data = cloned_state.deref().clone();
        form_onsubmit.emit(data);
    });
    
    html! {
        <div class={classes!("form-div")}>
            <h1>{"Login"}</h1>
            <form onsubmit={onsubmit}>
                <TextInput name="username" handle_onchange={username_changed} />
                <br />
                <TextInput name="password" handle_onchange={password_changed} />
                <br />
                <CustomButton label="Submit" />
            </form>
        </div>
    }
}

#[function_component(LoginComponent)]
pub fn login_component() -> Html {
    let navigator = use_navigator().unwrap();
    let custom_form_submit = Callback::from(move |data: LoginData| {
        log!("username is", &data.username);
        log!("password is", &data.password);
        let navigator = navigator.clone();

        wasm_bindgen_futures::spawn_local( async move {
            let url = format!("http://127.0.0.1:8000/api/v1/login");
            let form = [("username",data.username), ("password", data.password)];
            let client = reqwest::Client::builder()
                .build()
                .unwrap();

            let req = client.post(&url)
                .form(&form)
                .send()
                .await
                .unwrap()
                .text()
                .await;

            match req {
                Ok(req) => {
                    LocalStorage::set("Response", &req).unwrap();
                    navigator.push(&MainRoute::Home);
                },
                Err(req) => {
                    let err = req.to_string();
                    log!("{:?}",err);
                }
            }

                    
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
}

