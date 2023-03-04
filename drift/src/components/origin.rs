use std::fmt;
use gloo::console::log;
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew::function_component;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(EnumIter, Debug, PartialEq)]
pub enum Origin {
    CMET,
    KCL,
}

impl fmt::Display for Origin {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Origin::CMET => write!(f,"CMET"),
            Origin::KCL => write!(f,"KCL"),
        }
    }
}

#[derive(Properties, PartialEq)]
pub struct Props {
    pub handle_onchange: Callback<String>,
}

#[function_component(OriginSelector)]
pub fn origin_selector(props: &Props) -> Html {
    let mut origin_vec: Vec<Origin> = vec![];

    for uni in Origin::iter() {
        origin_vec.push(uni);
    };

    let handle_onchange = props.handle_onchange.clone();
    let onchange = Callback::from(move |event: Event| {
        let value = event
            .target()
            .unwrap()
            .unchecked_into::<HtmlInputElement>()
            .value();
        log!(&value);
        handle_onchange.emit(value);
    });    

    html! {
        <select id="origin" class={classes!("form-input")} onchange={onchange}>
            <option selected={true}>{"Select University"}</option>
            {
                origin_vec.iter().map(|uni|
                    html!{
                        <option value={format!("{}",uni)} >{format!("{}",uni)}</option>
                    }).collect::<Html>()
            }
        </select>
    }
}
