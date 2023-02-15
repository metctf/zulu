use serde::Deserialize;
use serde::Serialize;

#[derive(FromForm,Serialize,Deserialize,Debug,Default)]
pub struct Flag {
    pub flagid: String,
    pub challenge: String,
    pub challengeauthor: String,
    pub flagstring: String,
    pub points: u32,
}
