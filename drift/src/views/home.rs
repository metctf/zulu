use yew::prelude::*;

#[function_component(Home)]
pub fn home() -> Html {
    let onclick = Callback::from(move |_| {
        wasm_bindgen_futures::spawn_local( async move {
            let url = format!("http://127.0.0.1:8000/api/v1/leaderboard");

            let _body = reqwest::get(&url)
                .await
                .unwrap()
                .text()
                .await
                .unwrap();
            });
    });
    html! {
        <button {onclick}>{ "Click" }</button>
    }
}
