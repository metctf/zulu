use gloo_storage::{LocalStorage, Storage};
use reqwest::Client;
use yew::prelude::*;
use gloo::console::log;

use crate::forms::submit_flag::{FlagStringData,SubmitFlag};
use crate::components::challenge_list::{Challenge, ChallengeInfoList, AuthorsChallenges};
use crate::components::footer::Footer;

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

                let client = reqwest::Client::builder()
                    .build()
                    .unwrap();
                let url = format!("http://127.0.0.1:8000/api/v1/get_challenge/{}", search);
                let fetched_challenges: Vec<Challenge> = client.get(&url)
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
            <div class={classes!("search-div")}>
                <ChallengeInfoList challenge={(*challenges).clone()}/>
            </div>
        </>
    }
 
}


#[function_component(DisplayAuthorChallenge)]
pub fn challenge() -> Html {
    let challenges = use_state(|| vec![]);
    {
        let challenges = challenges.clone();
        use_effect_with_deps(move |_| {
            let challenges = challenges.clone();
            wasm_bindgen_futures::spawn_local(async move {

                let client = reqwest::Client::builder()
                    .build()
                    .unwrap();
                let jwt: String = LocalStorage::get("_AuthToken").expect("No auth token");
                let url = format!("http://127.0.0.1:8000/api/v1/get_author_challenge");
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
            <div class={classes!("search-div")}>
                <AuthorsChallenges challenge={(*challenges).clone()}/>
            </div>
        </>
    }
 
}

#[function_component(ChallengeTemplate)]
pub fn challenge(props: &Props) -> Html {

    let search = props.flag.clone();
    let custom_form_submit: Callback<FlagStringData> = Callback::from(move |data: FlagStringData| {
        let search = search.clone();
        log!("String is", &data.flagstring);

        wasm_bindgen_futures::spawn_local( async move {
            let url = format!("http://127.0.0.1:8000/api/v1/submit_challenge/");
            let form = [("name",search.clone()), ("flag", data.flagstring)];
            reqwest::Client::new()
                .post(&url)
                .form(&form)
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

    let search = props.flag.clone();
    /*
    let get_challenge_file = Callback::from(move |_| {
        let search = search.clone();
        wasm_bindgen_futures::spawn_local(async move {
            let search = search.clone();
            let client = reqwest::Client::builder()
                .build()
                .unwrap();

            let url = format!("http://127.0.0.1:8000/api/v1/static/challenge/{}", search);
            client.get(&url)
                .send()
                .await
                .unwrap();
        });
    });
    */
    let url = format!("http://127.0.0.1:8000/api/v1/static/challenge/{}", search);
    html! {
        <>
            <div class={classes!("challenge-div")}>
                <div class={classes!("challenge-title")}>
                    <h1>{&challenge.name}</h1>
                </div>
                <div class={classes!("challenge-description")}>
                    <p>{"Zulu is a CTF server currently in development by the Cardiff Met CTF society for use with our challenges produced by members of our society. It will allow us to keep track of the number of flags and points associated with them for each member of our society, which will be displayed on a society-wide leaderboard. Zulu will be designed to be university agnostic and highly configurable, allowing other university societies to use our software for their own uses if they so wish, with their own branding and such. Zulu is licensed under the GPLv3 and is free software."}</p>
                </div>
                <img src="../static/images/rust.png" class={classes!("image")} />
                <a class={classes!("download-button")} href={url.clone()}>
                    <button class={classes!("download-button")} >{"Click to Download Challenge "}</button>
                </a>
                <SubmitFlag onsubmit={custom_form_submit}/>
            </div>
            <Footer />
        </>
    }
}
