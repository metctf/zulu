use rocket::form::Form;
use rocket::State;
use super::super::structs::flag::Flag;
use super::super::structs::json::JsonResponse;
use rocket::serde::{Serialize, json::Json};
use rocket::response::{content, status};
use rocket::http::Status;
use super::super::connections::database::{Pool, create_flag};

#[post("/create_flag", data = "<flag>")]
pub async fn create_flag_api(pool: &State<Pool>, flag: Form<Flag>) -> status::Custom<Json<JsonResponse>> {
    let query = create_flag(&flag, pool).await;

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
            status::Custom(Status::Forbidden, Json(JsonResponse { id: String::from("") }))
        } 
    }
}
