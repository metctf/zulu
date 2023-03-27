use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub class: String,
    pub name: String,
    pub handle_onchange: Callback<String>,
}

#[derive(Properties, PartialEq)]
pub struct FileProps {
    pub class: String,
    pub name: String,
    pub handle_onchange: Callback<HtmlInputElement>,
}

#[function_component(TextInput)]
pub fn text_input(props: &Props) -> Html {
    let handle_onchange = props.handle_onchange.clone();
    let onchange = Callback::from(move |event: Event| {
        let value = event
            .target()
            .unwrap()
            .unchecked_into::<HtmlInputElement>()
            .value();
        handle_onchange.emit(value);
    });
    html! {
      <input type="text" class={classes!(props.class.clone())} name={props.name.clone()} onchange={onchange} placeholder={props.name.clone()} />
    }
}

#[function_component(FileInput)]
pub fn text_input(props: &FileProps) -> Html {
    let handle_onchange = props.handle_onchange.clone();
    let onchange = Callback::from(move |event: Event| {
        let value = event
            .target()
            .unwrap()
            .unchecked_into::<HtmlInputElement>();
        handle_onchange.emit(value);
    });
    html! {
      <input type="file" accept="*" class={classes!(props.class.clone())} name={props.name.clone()} onchange={onchange} placeholder={props.name.clone()} />
    }
}

#[function_component(PasswordInput)]
pub fn password_input(props: &Props) -> Html {
    let handle_onchange = props.handle_onchange.clone();
    let onchange = Callback::from(move |event: Event| {
        let value = event
            .target()
            .unwrap()
            .unchecked_into::<HtmlInputElement>()
            .value();
        handle_onchange.emit(value);
    });
    html! {
      <input type="password" class={classes!(props.class.clone())} name={props.name.clone()} onchange={onchange} placeholder={props.name.clone()} />
    }
}
