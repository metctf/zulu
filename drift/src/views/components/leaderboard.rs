use serde::Deserialize;
use yew::prelude::*;

#[derive(Clone, PartialEq, Deserialize)]
pub struct Leaderboard {
    pub user: String,
    pub score: u32,
}

#[derive(Properties, PartialEq)]
pub struct LeaderboardProps {
    pub users: Vec<Leaderboard>,
}

#[function_component(LeaderboardList)]
pub fn leaderboard_component(LeaderboardProps { users }: &LeaderboardProps) -> Html {
    users.iter().map(|user| html! {
        <div>{format!{"{}: {}",user.user, user.score}}</div>
    }).collect()
}
