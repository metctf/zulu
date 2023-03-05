use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub class: String,
    pub label: String,
}

#[function_component(CustomButton)]
pub fn custom_button(props: &Props) -> Html {
    html! {
      <button class={classes!(props.class.clone())}>{&props.label}</button>
    }
}
