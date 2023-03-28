pub mod challenge;
pub mod leaderboard;
pub mod login;
pub mod user;
pub mod origin;
pub mod json;

pub trait Verify {
    fn verify_password(&self, password: &str) -> bool;
    fn verify_flag(&self, flag: &str) -> bool;
}
