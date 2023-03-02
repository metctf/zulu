use rocket::{State, serde::json::Json};

use crate::connections::database::{Pool, submit_flag};

use rocket::response::{content, status};
use rocket::http::Status;
use crate::structs::json::JsonResponse;

#[get("/submit_challenge/<id>")]
pub async fn submit_flag_api(pool: &State<Pool>, id: String) -> status::Custom<Json<bool>> {
    let query = submit_flag(id, pool).await;
    match query {
        Ok(_) => status::Custom(Status::Ok, Json(true)),
        Err(_) => status::Custom(Status::NotFound, Json(false))
    }
}
