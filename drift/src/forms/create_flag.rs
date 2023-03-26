use yew::prelude::*;
use yew_router::prelude::*;
use std::ops::Deref;

use crate::components::text_input::TextInput;
use crate::components::custom_button::CustomButton;
use crate::components::number_input::NumberInput;
use crate::router::MainRoute;

#[derive(Default, Clone)]
pub struct FlagData {
    pub name: String,
    pub author: String,
    pub flag: String,
    pub points: u32,
}

#[derive(Properties, PartialEq)]
pub struct Props {
    pub onsubmit: Callback<FlagData>,
}

#[function_component(CreateFlag)]
pub fn create_flag(props: &Props) -> Html {
    let state = use_state(|| FlagData::default());
    let cloned_state = state.clone();
    let name_changed = Callback::from(move |name| {
        let mut data = cloned_state.deref().clone();
        data.name = name;
        cloned_state.set(data);
    });

    let cloned_state = state.clone();
    let author_changed = Callback::from(move |author| {
        let mut data = cloned_state.deref().clone();
        data.author = author;
        cloned_state.set(data);
    });

    let cloned_state = state.clone();
    let flag_changed = Callback::from(move |flag| {
        let mut data = cloned_state.deref().clone();
        data.flag = flag;
        cloned_state.set(data);
    });

    let cloned_state = state.clone();
    let points_changed = Callback::from(move |points| {
        let mut data = cloned_state.deref().clone();
        data.points = points;
        cloned_state.set(data);
    });

    let navigator = use_navigator().unwrap();
    let form_onsubmit = props.onsubmit.clone();
    let cloned_state = state.clone();
    let onsubmit = Callback::from(move |event: SubmitEvent| {
        event.prevent_default();
        let data = cloned_state.deref().clone();
        form_onsubmit.emit(data);
        navigator.push(&MainRoute::Home);
    });

    html! {
        <div class={classes!("form-div")}>
            <h1>{"Create a Flag"}</h1>
            <form onsubmit={onsubmit}>
                <TextInput name="name" class="form-input" handle_onchange={name_changed} />
                <br />
                <TextInput name="author" class="form-input" handle_onchange={author_changed} />
                <br />
                <TextInput name="flag" class="form-input" handle_onchange={flag_changed} />
                <br />
                <NumberInput name="points" class="form-input" handle_onchange={points_changed} />
                <br />
                <CustomButton label="Submit" class="button" />
            </form>
        </div>
    }
}
