use yew::prelude::*;
use std::ops::Deref;

use crate::components::text_input::TextInput;
use crate::components::custom_button::CustomButton;

#[derive(Default, Clone)]
pub struct FlagStringData {
    pub flagname: String,
    pub flagstring: String,
}

#[derive(Properties, PartialEq)]
pub struct SubmitProps {
    pub onsubmit: Callback<FlagStringData>,
}


#[function_component(SubmitFlag)]
pub fn submit_flag(props: &SubmitProps) -> Html {

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
        <>
            <div class={classes!("submit-challenge")}>
                <form onsubmit={onsubmit} class={classes!("string-input")}>
                    <TextInput name="flagstring" class="string-input" handle_onchange={flagstring_changed} />
                    <CustomButton label="Submit Flag"  class="submit-button" />
                </form>
            </div>
        </>
    }
}
