use serde::Deserialize;
use yew::prelude::*;
use std::ops::Deref;

use crate::components::text_input::{PasswordInput, TextInput};
use crate::components::custom_button::CustomButton;
use crate::components::origin::OriginSelector;

#[derive(Deserialize)]
pub struct Jwt {
    pub jwt: String,
}

#[derive(Default, Clone)]
pub struct LoginData {
    pub username: String,
    pub password: String,
    pub origin: String,
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
            <h1>{"Login"}</h1>
            <form onsubmit={onsubmit}>
                <TextInput name="username" class="form-input" handle_onchange={username_changed} />
                <br />
                <PasswordInput name="password" class="form-input" handle_onchange={password_changed} />
                <br />
                <OriginSelector handle_onchange={origin_changed}/>
                <br />
                <CustomButton label="Submit" class="button" />
            </form>
        </div>
    }
}

