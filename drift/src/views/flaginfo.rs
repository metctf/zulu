use yew::prelude::*;
use crate::views::flag::{Flag, FlagInfo};
use gloo_net::http::Request;

#[function_component(DisplayFlag)]
pub fn flag() -> Html {
    let flags = use_state(|| vec![]);
    {
        let flags = flags.clone();
        use_effect_with_deps(move |_| {
            let flags = flags.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let fetched_flags: Vec<Flag> = Request::get("http://127.0.0.1:8000/api/v1/get_flag/1")
                    .send()
                    .await
                    .unwrap()
                    .json()
                    .await
                    .unwrap();
                flags.set(fetched_flags);
            });
            || ()
        }, ());
    }
    html! {
        <>
            <div>
                <FlagInfo flags={(*flags).clone()} />
            </div>
        </>
    }
 
}
