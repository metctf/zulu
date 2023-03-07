use rocket::{State, serde::json::Json};

use crate::connections::database::{Pool, submit_challenge};

use rocket::response::status;
use rocket::http::Status;

#[get("/submit_challenge/<id>")]
pub async fn submit_challenge_api(pool: &State<Pool>, id: String) -> status::Custom<Json<bool>> {
    let query = submit_challenge(id, pool).await;
    match query {
        Ok(_) => status::Custom(Status::Ok, Json(true)),
        Err(_) => status::Custom(Status::NotFound, Json(false))
    }
}
