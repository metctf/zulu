use yew::prelude::*;
use std::ops::Deref;

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
       <form onsubmit={onsubmit}>
            <TextInput name="username" handle_onchange={username_changed} />
            <br />
            <TextInput name="password" handle_onchange={password_changed} />
            <br />
            <CustomButton label="Submit" />
        </form>
    }
}
