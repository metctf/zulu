use rocket::form::Form;
use rocket::State;
use crate::structs::challenge::Challenge;
use super::super::structs::json::JsonResponse;
use rocket::serde::json::Json;
use rocket::response::status;
use rocket::http::Status;
use super::super::connections::database::{Pool, create_challenge};

#[post("/create_challenge", data = "<challenge>")]
pub async fn create_challenge_api(pool: &State<Pool>, challenge: Form<Challenge>) -> status::Custom<Json<JsonResponse>> {
    let query = create_challenge(&challenge, pool).await;

    match query {
        Ok(query) => {
            info!("Registered new flag: {}", query);
            let resp = JsonResponse {
                id: String::from(query)
            };
            status::Custom(Status::Ok, Json(resp))
        },
        Err(query) =>{
            error!("Couldn't create flag: {}", query);
            status::Custom(Status::InternalServerError, Json(JsonResponse { id: String::from("") }))
        } 
    }
}
