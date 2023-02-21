use yew::prelude::*;
use std::ops::Deref;

use super::components::text_input::TextInput;
use super::components::custom_button::CustomButton;

#[derive(Default, Clone)]
pub struct FlagStringData {
    pub flagstring: String,
}

#[derive(Properties, PartialEq)]
pub struct Props {
    pub onsubmit: Callback<FlagStringData>,
}


#[function_component(SubmitFlag)]
pub fn submit_flag(props: &Props) -> Html {

    let state = use_state(|| FlagStringData::default());

    let cloned_state = state.clone();
    let flagstring_changed = Callback::from(move |flagstring| {
        let mut data = cloned_state.deref().clone();
        data.flagstring = flagstring;
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
            <h1>{"Submit Flag"}</h1>
            <form onsubmit={onsubmit}>
            <TextInput name="flagstring" class="form-input" handle_onchange={flagstring_changed} />
            <br />
            <CustomButton label="Submit"  />
            </form>
        </div>
    }
}
