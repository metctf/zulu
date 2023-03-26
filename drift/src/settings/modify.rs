use yew::prelude::*;
use yew_router::prelude::*;
use gloo::console::log;
use gloo::storage::LocalStorage;
use gloo_storage::Storage;

use crate::components::top_bar::{NavBar, Tab};
use crate::components::footer::Footer;
use crate::forms::register_form::{RegisterForm, RegisterData};
use crate::MainRoute;


#[function_component(ModifyComponent)]
pub fn modify_component() -> Html {
    let navigator = use_navigator().unwrap();
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
                    let jwt: String = LocalStorage::get("_AuthToken").unwrap();

                    client.post(&url)
                        .header("auth", jwt)
                        .form(&form)
                        .send()
                        .await
                        .unwrap(); //Getting an error here
                });
            });

            let delete = Callback::from(move |_| {
            let navigator = navigator.clone();
                wasm_bindgen_futures::spawn_local(async move{
                    let url = format!("http://127.0.0.1:8000/api/v1/remove");
                    let client = reqwest::Client::new();

                    client.delete(&url)
                        .send()
                        .await
                        .unwrap();
                    
                    LocalStorage::clear();
                    navigator.push(&MainRoute::Home);
                    if let Some(window) = web_sys::window() {
                        window.location().reload().unwrap();
                    };
                });
            });

            html! {
                <>
                    <NavBar tab={Tab::Authorized}/>
                    <RegisterForm name={"Update User Details"} onsubmit={custom_form_submit}>
                        <br />
                        <button class={classes!("button")} style="background-color:red;" onclick={delete}>{"Delete"}</button>
                    </RegisterForm>
                    <Footer />
                </>
            }
}
