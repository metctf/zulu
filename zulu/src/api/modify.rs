use std::str::FromStr;

use rocket::form::Form;
use rocket::State;

use crate::auth::accesslevel::AccessLevel;
use crate::structs::origin::Origin;
use crate::structs::user::User;
use super::super::auth::jwt::JwtToken;
use super::super::connections::database::{Pool, modify_user, get_user_info};

use rocket::serde::{Serialize, json::Json};
use rocket::response::{content, status};
use rocket::http::Status;
use super::super::structs::json::JsonResponse;

#[post("/modify", data = "<user>")]
pub async fn modify(pool: &State<Pool>, token: JwtToken, user: Form<User>) -> status::Custom<Json<JsonResponse>>{
    let query = modify_user(&user, token, pool).await;
    /*
     * Send a 200 code if successful
     */
    match query{
        Ok(_query) => {
            let resp = JsonResponse {
                id: String::from(_query)
            };
            return status::Custom(Status::Ok, Json(resp))
        }

        Err(_) => {
            status::Custom(Status::NotFound, Json(JsonResponse { id: String::from("") }))
        }
    }

}

#[get("/")]
pub async fn display_user_info(pool: &State<Pool>, token: JwtToken) -> status::Custom<Json<User>>{
    /*
     * Return info as json
     */
    let query = get_user_info(token, pool).await;

    match query {
        Ok(query) => status::Custom(Status::Ok, Json(query)),
        Err(_) => {
            let user = User { // create empty user struct to represent no match
                accountid: String::from(""),
                username: String::from(""),
                firstname: String::from(""),
                lastname: String::from(""),
                password: String::from(""),
                origin: Origin::from_str("Internal").unwrap(),
                solves: 0,
                accesslevel: AccessLevel::from_str("User").unwrap(),
            };
            status::Custom(Status::InternalServerError, Json(user))
        }
    }
}
