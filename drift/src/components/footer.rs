use yew::prelude::*;
use gloo::utils::document;
use gloo_storage::LocalStorage;
use gloo::storage::Storage;

#[function_component(Footer)]
pub fn footer() -> Html {
    let on_click = Callback::from(|_: MouseEvent| {
            let root_element = document().document_element().unwrap();
            let theme = root_element.get_attribute("theme").unwrap();

            let new_theme: String;

            if theme.eq("light") {
                new_theme = format!("dark");
            }else{
                new_theme = format!("light");
            }
            root_element.set_attribute("theme", &new_theme).unwrap();
            LocalStorage::set("theme", &new_theme).unwrap();
    });

    html! {
        <div class={classes!("footer")}>
            <p class={classes!("label")}>{"Light"}</p>
            <label class={classes!("switch", "label")}>
                <input type="checkbox" onclick={on_click} />
                <span class="slider round" id="slider"></span>
            </label>
            <p class={classes!("label")}>{"Dark"}</p>
            <a href="https://github.com/metctf/zulu" class={classes!("zulu-github")}>{"Zulu CardiffMet CTF Society 2023 "}</a>
        </div>
    }
}
