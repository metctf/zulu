use yew::prelude::*;
use std::ops::Deref;

use crate::components::text_input::TextInput;

#[derive(Default, Clone)]
pub struct SearchData {
    pub searchterm: String,
}

#[derive(Properties, PartialEq)]
pub struct Props {
    pub onsubmit: Callback<SearchData>,
}


#[function_component(SearchBar)]
pub fn search_bar(props: &Props) -> Html {
    let state = use_state(|| SearchData::default());

    let cloned_state = state.clone();
    let searchterm_changed = Callback::from(move |searchterm| {
        let mut data = cloned_state.deref().clone();
        data.searchterm = searchterm;
        cloned_state.set(data);
    });

    let form_onsubmit = props.onsubmit.clone();
    let cloned_state = state.clone();
    let onsubmit = Callback::from(move |event: SubmitEvent| {
        event.prevent_default();
        let data = cloned_state.deref().clone();
        form_onsubmit.emit(data);
    });
 
    html!(
        <>
        <form onsubmit={onsubmit} style={"display: inline-block; width: 100%;"}>
            <TextInput name="Search" class="wide" handle_onchange={searchterm_changed} />
            <button class={classes!("search")} >{""}</button>
        </form>
        </>
        )
}
