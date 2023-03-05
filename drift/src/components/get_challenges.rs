use yew::prelude::*;
use crate::components::challenge_list::{Challenge, ChallengeInfoList};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub flag: String,
}

#[function_component(GetChallenges)]
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
            <div>
                <ChallengeInfoList challenge={(*challenges).clone()}/>
            </div>
        </>
    }
 
}

