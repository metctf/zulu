use serde::Deserialize;
use yew::prelude::*;
use gloo_storage::errors::StorageError;
use yew_router::prelude::*;
use gloo::console::log;
use gloo::storage::LocalStorage;
use gloo_storage::Storage;

use crate::components::top_bar::{NavBar, Tab};
use crate::components::footer::Footer;
use crate::MainRoute;

use crate::forms::login_form::{Login, LoginData};

#[derive(Deserialize)]
pub struct Jwt {
    pub jwt: String,
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
            let form = [("username",data.username), ("password", data.password), ("origin", data.origin)];
            let client = reqwest::Client::builder()
                .build()
                .unwrap();

            let req = client.post(&url)
                .form(&form)
                .send()
                .await
                .unwrap()
                .json::<Jwt>()
                .await;

            match req {
                Ok(req) => {
                    LocalStorage::set("_AuthToken", &req.jwt).unwrap();
                    navigator.push(&MainRoute::Home);
                },
                Err(req) => {
                    let err = req.to_string();
                    log!(err);
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
                <Footer />
            </>
        }
}

