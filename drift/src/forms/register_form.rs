use yew::prelude::*;
use std::ops::Deref;

use crate::components::text_input::{PasswordInput, TextInput};
use crate::components::custom_button::CustomButton;
use crate::components::origin::OriginSelector;

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

#[function_component(RegisterForm)]
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
                <PasswordInput name="password" class="form-input" handle_onchange={password_changed} />
                <br />
                <OriginSelector handle_onchange={origin_changed}/>
                <br />
                <CustomButton label="Submit" class="button"  />
            </form>
            { for props.children.iter() }
        </div>
    }
}

