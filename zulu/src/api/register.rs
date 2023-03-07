use rocket::form::Form;
use rocket::State;
use crate::structs::{user::User, json::JsonResponse};
use super::super::connections::database::{Pool, register_account};

use rocket::serde::json::Json;
use rocket::response::status;
use rocket::http::Status;

#[post("/register", data = "<user>")]
pub async fn register(pool: &State<Pool>, user: Form<User>) -> status::Custom<Json<JsonResponse>> {

    let reg_user = &user; // make clone of form to prevent copy errors

    let query = register_account(pool, reg_user).await;
    /*
     * Check if the user was entered correctly into the database, if any errors
     * occur handle them
     */
    match query {
        Ok(_query) => {
            info!("Registered new user: {}", &user.username);
            let resp = JsonResponse {
                id: String::from(_query)
            };
            status::Custom(Status::Ok, Json(resp))
        },
        Err(_) => {
            error!("Couldn't register user: {}", &user.username);
            status::Custom(Status::InternalServerError, Json(JsonResponse { id: String::from("") }))
        } 
    }
}

