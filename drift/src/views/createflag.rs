use yew::prelude::*;
use std::ops::Deref;

use super::components::text_input::TextInput;
use super::components::custom_button::CustomButton;
use crate::views::components::number_input::NumberInput;

#[derive(Default, Clone)]
pub struct FlagData {
    pub flagid: u32,
    pub challenge: String,
    pub challengeauthor: String,
    pub flagstring: String,
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
    let flagid_changed = Callback::from(move |flagid| {
        let mut data = cloned_state.deref().clone();
        data.flagid = flagid;
        cloned_state.set(data);
    });

    let cloned_state = state.clone();
    let challenge_changed = Callback::from(move |challenge| {
        let mut data = cloned_state.deref().clone();
        data.challenge = challenge;
        cloned_state.set(data);
    });

    let cloned_state = state.clone();
    let challengeauthor_changed = Callback::from(move |challengeauthor| {
        let mut data = cloned_state.deref().clone();
        data.challengeauthor = challengeauthor;
        cloned_state.set(data);
    });

    let cloned_state = state.clone();
    let flagstring_changed = Callback::from(move |flagstring| {
        let mut data = cloned_state.deref().clone();
        data.flagstring = flagstring;
        cloned_state.set(data);
    });

    let cloned_state = state.clone();
    let points_changed = Callback::from(move |points| {
        let mut data = cloned_state.deref().clone();
        data.points = points;
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
            <h1>{"Create a Flag"}</h1>
            <form onsubmit={onsubmit}>
                <NumberInput name="flagid" class="form-input" handle_onchange={flagid_changed} />
                <br />
                <TextInput name="challenge" class="form-input" handle_onchange={challenge_changed} />
                <br />
                <TextInput name="challengeauthor" class="form-input" handle_onchange={challengeauthor_changed} />
                <br />
                <TextInput name="flagstring" class="form-input" handle_onchange={flagstring_changed} />
                <br />
                <NumberInput name="points" class="form-input" handle_onchange={points_changed} />
                <br />
                <CustomButton label="Submit" />
            </form>
        </div>
    }
}
