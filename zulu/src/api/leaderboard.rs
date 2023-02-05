use rocket::State;
use rocket::serde::json::Json;
use crate::structs::leaderboard::Leaderboard;
use super::super::connections::database::{Pool,get_top_30};

#[get("/leaderboard")]
pub async fn leaderboard(pool: &State<Pool>) -> Json<Vec<Leaderboard>>{
    let records:Result<Vec<Leaderboard>,sqlx::Error> = get_top_30(pool).await; 
    match records {
        Ok(records) => Json(records),
        Err(_) => panic!("no records")
    }
}
