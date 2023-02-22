use yew::prelude::*;
use serde::Deserialize;

#[derive(Clone, PartialEq, Deserialize, Default)]
pub struct Flag {
    pub flagid: String,
    pub challenge: String,
    pub challengeauthor: String,
    pub flagstring: String,
    pub points: u32,
}

#[derive(Properties, PartialEq)]
pub struct FlagProps {
    pub flag: Flag,
}

#[function_component(FlagInfo)]
pub fn flag_info(FlagProps {flag}: &FlagProps) -> Html {
        html! {
            <>
            <div class={classes!("leaderboard")}>
                <p>{&flag.challenge}</p>
                <p>{&flag.challengeauthor}</p>
                <p>{&flag.points.to_string()}</p>
            </div>
            </>
        }
}
