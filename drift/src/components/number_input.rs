use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub class: String,
    pub name: String,
    pub handle_onchange: Callback<u32>,
}

#[function_component(NumberInput)]
pub fn text_input(props: &Props) -> Html {
    let handle_onchange = props.handle_onchange.clone();
    let onchange = Callback::from(move |event: Event| {
        let value = event
            .target()
            .unwrap()
            .unchecked_into::<HtmlInputElement>()
            .value_as_number();
        handle_onchange.emit(value as u32);
    });
    html! {
      <input type="number" class={classes!(props.class.clone())} name={props.name.clone()} onchange={onchange} placeholder={props.name.clone()} />
    }
}
