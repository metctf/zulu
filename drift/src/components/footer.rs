use yew::prelude::*;
use gloo::utils::document;
use gloo_storage::LocalStorage;
use gloo::storage::Storage;

fn theme_select() -> bool {
    let theme: Result<String,_> = LocalStorage::get("theme");

    // Check what the theme is a returned value of false is light mode 
    // and true is dark mode
    match theme {
        Ok(theme) => {
            if theme.eq("light") {
                false
            } else {
                true 
            }
        },
        Err(_) => {
            false
        }
    }
}

#[function_component(Footer)]
pub fn footer() -> Html {
    let is_checked: bool = theme_select();

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
                <input type="checkbox" checked={is_checked} onclick={on_click} />
                <span class="slider round" ></span>
            </label>
            <p class={classes!("label")}>{"Dark"}</p>
            <a href="https://github.com/metctf/zulu" class={classes!("zulu-github")}>{"Zulu - Cardiff Met CTF Society 2023 "}</a>
        </div>
    }
}
