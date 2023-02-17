use yew::prelude::*;
use serde::Deserialize;

#[derive(Clone, PartialEq, Deserialize)]
pub struct Flag {
    pub challenge: String,
    pub challengeauthor: String,
    pub points: u32,
}

#[derive(Properties, PartialEq)]
pub struct FlagProps {
    pub flags: Vec<Flag>,
}

#[function_component(FlagInfo)]
pub fn flag_info(FlagProps {flags}: &FlagProps) -> Html {
        flags.iter().map(|flags| html! {
            <>
            <div>
                <p>{&flags.challenge}</p>
                <p>{&flags.challengeauthor}</p>
                <p>{&flags.points.to_string()}</p>
            </div>
            </>
        }).collect()
}
