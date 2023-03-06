use serde::Deserialize;
use yew::prelude::*;

#[derive(Clone, PartialEq, Deserialize)]
pub struct Leaderboard {
    pub username: String,
    pub solves: u32,
}

#[derive(Properties, PartialEq)]
pub struct LeaderboardProps {
    pub users: Vec<Leaderboard>,
}

#[function_component(LeaderboardList)]
pub fn leaderboard_component(LeaderboardProps { users }: &LeaderboardProps) -> Html {
    users.iter().map(|user| html! {
        <div class={classes!("leaderboard")}>{format!{"{}: {}",user.username, user.solves}}</div>
    }).collect()
}
