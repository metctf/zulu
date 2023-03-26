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
            <div class={classes!("search-result")}>
                <p class={classes!("left", "one")}>{format!("{} by {}", &challenge.name, &challenge.author)}</p>
                <p class={classes!("left", "two")}>{format!("Awards {} points", &challenge.points.to_string())}</p>
                <Link<MainRoute> to={MainRoute::Challenge { id: challenge.name.clone() }} classes={classes!("three")}>{"Click to display challenge"}</Link<MainRoute>>
            </div>
            </>
        }).collect()
}

#[function_component(AuthorsChallenges)]
pub fn manage_challenge_list(ChallengeProps {challenge}: &ChallengeProps) -> Html {
    challenge.iter().map(|challenge| html! {
        <>
        <div class={classes!("search-result")}>
            <p class={classes!("left", "one")}>{format!("{}", &challenge.name)}</p>
            <p class={classes!("left", "two")}>{format!("Awards {} points", &challenge.points.to_string())}</p>
            <Link<MainRoute> to={MainRoute::UploadChallenge { id: challenge.name.clone() }} classes={classes!("three")}>{"Click to manage Challenge"}</Link<MainRoute>>
        </div>
        </>
    }).collect()
}


