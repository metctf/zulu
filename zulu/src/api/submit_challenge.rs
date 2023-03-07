use rocket::{State, serde::json::Json};

use crate::connections::database::{Pool, submit_challenge};

use crate::structs::challenge::SubmitChallenge;

use rocket::response::status;
use rocket::http::Status;
use rocket::form::Form;

#[post("/submit_challenge", data = "<challenge>")]
pub async fn submit_challenge_api(pool: &State<Pool>, challenge: Form<SubmitChallenge>) -> status::Custom<Json<bool>> {
    let query = submit_challenge(challenge, pool).await;
    match query {
        Ok(_) => status::Custom(Status::Ok, Json(true)),
        Err(_) => status::Custom(Status::NotFound, Json(false))
    }
}
