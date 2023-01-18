use rocket::State;
use super::super::auth::user::Leaderboard;
use super::super::connections::database::{Pool,get_top_30};

#[get("/leaderboard")]
pub async fn leaderboard(pool: &State<Pool>) -> String{
    let records:Result<Vec<Leaderboard>,sqlx::Error> = get_top_30(pool).await; 
    format!("{:?}",records)
}
