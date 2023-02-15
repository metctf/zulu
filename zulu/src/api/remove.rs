use rocket::State;
use super::super::auth::jwt::JwtToken;
use super::super::connections::database::{Pool,delete_account};

use rocket::serde::{Serialize, json::Json};
use rocket::response::{content, status};
use rocket::http::Status;
use crate::structs::json::JsonResponse;

#[delete("/remove")]
pub async fn remove_account(pool: &State<Pool>, token: JwtToken) -> status::Custom<Json<JsonResponse>> {

    let id = token.accountid.clone();

    let query = delete_account(pool, token).await;
    match query{
        Ok(query) => {
            if query{
                let resp = JsonResponse {
                    id: String::from(id)
                };
                return status::Custom(Status::Ok, Json(resp))
            }
            return status::Custom(Status::Forbidden, Json(JsonResponse { id: String::from("") }))
        },
        Err(_) => return status::Custom(Status::Forbidden, Json(JsonResponse { id: String::from("") }))
    }
}
