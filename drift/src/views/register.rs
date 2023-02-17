use gloo::console::log;
use yew::prelude::*;
use yew_router::prelude::use_navigator;
use std::ops::Deref;

use crate::router::MainRoute;

use super::components::text_input::TextInput;
use super::components::custom_button::CustomButton;
use crate::views::components::top_bar::{NavBar, Tab};

#[derive(Default, Clone)]
pub struct RegisterData {
    pub username: String,
    pub firstname: String,
    pub lastname: String,
    pub password: String,
    pub origin: String,
}

#[derive(Properties, PartialEq)]
pub struct Props {
    pub name: String,
    pub onsubmit: Callback<RegisterData>,
    #[prop_or_default]
    pub children: Children,
}

#[function_component(Register)]
pub fn register(props: &Props) -> Html {
    let state = use_state(|| RegisterData::default());

    let cloned_state = state.clone();
    let username_changed = Callback::from(move |username| {
        let mut data = cloned_state.deref().clone();
        data.username = username;
        cloned_state.set(data);
    });

    let cloned_state = state.clone();
    let firstname_changed = Callback::from(move |firstname| {
        let mut data = cloned_state.deref().clone();
        data.firstname = firstname;
        cloned_state.set(data);
    });

    let cloned_state = state.clone();
    let lastname_changed = Callback::from(move |lastname| {
        let mut data = cloned_state.deref().clone();
        data.lastname = lastname;
        cloned_state.set(data);
    });

    let cloned_state = state.clone();
    let password_changed = Callback::from(move |password| {
        let mut data = cloned_state.deref().clone();
        data.password = password;
        cloned_state.set(data);
    });

    let cloned_state = state.clone();
    let origin_changed = Callback::from(move |origin| {
        let mut data = cloned_state.deref().clone();
        data.origin = origin;
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
            <h1>{props.name.clone()}</h1>
            <form onsubmit={onsubmit}>
                <TextInput name="username" class="form-input" handle_onchange={username_changed} />
                <br />
                <TextInput name="firstname" class="form-input" handle_onchange={firstname_changed} />
                <br />
                <TextInput name="lastname" class="form-input" handle_onchange={lastname_changed} />
                <br />
                <TextInput name="password" class="form-input" handle_onchange={password_changed} />
                <br />
                <TextInput name="origin" class="form-input" handle_onchange={origin_changed} />
                <br />
                <CustomButton label="Submit"  />
            </form>
            { for props.children.iter() }
        </div>
    }
}

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
                <Register name={"Register"} onsubmit={custom_form_submit} />
            </>
        }
}
