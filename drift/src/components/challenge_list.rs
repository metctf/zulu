use yew::prelude::*;
use yew_router::prelude::*;
use crate::MainRoute;
use serde::Deserialize;

#[derive(Clone, PartialEq, Deserialize, Default)]
pub struct Challenge {
    pub id: String,
    pub name: String,
    pub author: String,
    pub flag: String,
    pub points: u32,
}

#[derive(Properties, PartialEq)]
pub struct ChallengeProps {
    pub challenge: Vec<Challenge>,
}

#[function_component(ChallengeInfoList)]
pub fn challenge_info(ChallengeProps {challenge}: &ChallengeProps) -> Html {
    challenge.iter().map(|challenge| html! {
            <>
            <div class={classes!("leaderboard")}>
                <p>{&challenge.name}</p>
                <p>{&challenge.author}</p>
                <p>{&challenge.points.to_string()}</p>
                <Link<MainRoute> to={MainRoute::Challenge { id: challenge.name.clone() }}>{"Click to display challenge"}</Link<MainRoute>>
            </div>
            </>
        }).collect()
}
