use gloo_storage::{LocalStorage, Storage};
use yew::prelude::*;
use crate::views::challenge::{Challenge, ChallengeInfo};
use gloo::console::log;

use crate::views::submitflag::{FlagStringData,SubmitFlag};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub flag: String,
}

#[function_component(DisplayChallenge)]
pub fn challenge(props: &Props) -> Html {
    let challenges = use_state(|| vec![]);
    let search = props.flag.clone();
    {
        let challenges = challenges.clone();
        use_effect_with_deps(move |_| {
            let challenges = challenges.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let jwt: String = LocalStorage::get("_AuthToken").unwrap();

                let client = reqwest::Client::builder()
                    .build()
                    .unwrap();
                let url = format!("http://127.0.0.1:8000/api/v1/get_challenge/{}", search);
                let fetched_challenges: Vec<Challenge> = client.get(&url)
                    .header("auth", jwt)
                    .send()
                    .await
                    .unwrap()
                    .json()
                    .await
                    .unwrap();
                challenges.set(fetched_challenges);
            });
            || ()
        }, ());
    }
    html! {
        <>
            <div>
                <ChallengeInfo challenge={(*challenges).clone()}/>
            </div>
        </>
    }
 
}

#[function_component(ChallengeTemplate)]
pub fn challenge(props: &Props) -> Html {

    let custom_form_submit = Callback::from(|data: FlagStringData| {
        log!("String is", &data.flagstring);

        wasm_bindgen_futures::spawn_local( async move {
            let url = format!("http://127.0.0.1:8000/api/v1/submit_flag/{}", &data.flagstring);
            reqwest::Client::new()
                .get(&url)
                .send()
                .await
                .unwrap();

        });
    });

    let challenge = use_state(|| Challenge::default());
    let search = props.flag.clone();
    {
        let challenge = challenge.clone();
        use_effect_with_deps(move |_| {
            let challenge = challenge.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let client = reqwest::Client::builder()
                    .build()
                    .unwrap();
                let url = format!("http://127.0.0.1:8000/api/v1/get_single_challenge/{}", search);
                let fetched_challenge: Challenge = client.get(&url)
                    .send()
                    .await
                    .unwrap()
                    .json()
                    .await
                    .unwrap();
                challenge.set(fetched_challenge);
            });
            || ()
        }, ());
    }
    html! {
        <>
            <div class={classes!("form-div")}>
                <h1>{&challenge.name}</h1>
                <p>{"lil desription of the boxes"}</p>
                <SubmitFlag onsubmit={custom_form_submit}/>
            </div>
        </>
    }
}
