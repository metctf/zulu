use rocket::form::Form;
use rocket::State;
use crate::structs::challenge::Challenge;
use super::super::structs::json::JsonResponse;
use rocket::serde::json::Json;
use rocket::response::status;
use rocket::http::Status;
use rocket::data::Data;
use super::super::connections::database::{Pool, create_challenge};
use super::super::connections::filesystem::create_challenge_file;

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

#[post("/upload_challenge/<challenge>", data = "<file>")]
pub async fn upload_challenge_api(challenge: String, file: Data<'_>) -> Status {
    let query = create_challenge_file(challenge, file).await;


    match query {
        Ok(_) => {
            info!("Created new file for challenge");
            Status::Ok
        },
        Err(query) =>{
            error!("Couldn't write file for challenge: {}", query);
            Status::InternalServerError
        } 
    }
}
