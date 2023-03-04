use yew::prelude::*;
use crate::components::leaderboard::{LeaderboardList, Leaderboard};
use gloo_net::http::Request;

#[function_component(Home)]
pub fn home() -> Html {
    let users = use_state(|| vec![]);
    {
        let users = users.clone();
        use_effect_with_deps(move |_| {
            let users = users.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let fetched_users: Vec<Leaderboard> = Request::get("http://127.0.0.1:8000/api/v1/leaderboard")
                    .send()
                    .await
                    .unwrap()
                    .json()
                    .await
                    .unwrap();
                users.set(fetched_users);
            });
            || ()
        }, ());
    }
    html! {
        <>
            <div class={classes!("leaderboard-div")}>
                <h1>{"Global Leaderboard (Top 30 pwners)"}</h1>
                <LeaderboardList users={(*users).clone()} />
            </div>
        </>
    }
}
