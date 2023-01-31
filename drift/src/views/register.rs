use yew::prelude::*;
use std::ops::Deref;

use super::components::text_input::TextInput;
use super::components::custom_button::CustomButton;

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
    pub onsubmit: Callback<RegisterData>,
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
       <form onsubmit={onsubmit}>
            <TextInput name="username" handle_onchange={username_changed} />
            <TextInput name="firstname" handle_onchange={firstname_changed} />
            <TextInput name="lastname" handle_onchange={lastname_changed} />
            <TextInput name="password" handle_onchange={password_changed} />
            <TextInput name="origin" handle_onchange={origin_changed} />
            <CustomButton label="Submit" />
        </form>
    }
}
