use gloo::console::log;
use yew::prelude::*;
use yew_router::prelude::use_navigator;

use crate::router::MainRoute;
use crate::forms::register_form::{RegisterForm, RegisterData};
use crate::components::top_bar::{NavBar, Tab};
use crate::components::footer::Footer;

#[function_component(RegisterComponent)]
pub fn register_component() -> Html{
    let navigator = use_navigator().unwrap();
    let custom_form_submit = Callback::from( move |data: RegisterData| {
            log!("username is", &data.username);
            log!("password is", &data.password);
            let navigator = navigator.clone();

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
                navigator.push(&MainRoute::Home);
            });
        });
        
        html! {
            <>
                <NavBar tab={Tab::Unauthorized}/>
                <RegisterForm name={"Register"} onsubmit={custom_form_submit} />
                <Footer />
            </>
        }
}
