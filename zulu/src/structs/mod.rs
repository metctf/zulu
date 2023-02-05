pub mod flag;
pub mod leaderboard;
pub mod login;
pub mod user;
pub mod origin;

pub trait Verify {
    fn verify_password(&self, password: &str) -> bool;
}

