use serde::Deserialize;
use serde::Serialize;

#[derive(Debug,Serialize,Deserialize)]
pub struct Leaderboard{
    pub username: String,
    pub flagquantity: u32,
}

