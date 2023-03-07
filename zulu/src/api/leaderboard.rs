use rocket::State;
use crate::structs::leaderboard::Leaderboard;
use super::super::connections::database::{Pool,get_top_30};
use rocket::serde::json::Json;
use rocket::response::status;
use rocket::http::Status;

#[get("/leaderboard")]
pub async fn leaderboard(pool: &State<Pool>) -> status::Custom<Json<Vec<Leaderboard>>> {
    let records = get_top_30(pool).await; 
    match records {
        Ok(records) => status::Custom(Status::Ok, Json(records)),
        Err(_) => panic!("no records")
    }
}
