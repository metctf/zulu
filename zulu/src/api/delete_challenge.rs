use rocket::State;
use crate::auth::jwt::JwtToken;
use super::super::connections::database::{Pool,remove_challenge};
use rocket::response::{content, status};
use rocket::http::Status;
use super::super::structs::json::JsonResponse;
use rocket::serde::{Serialize, json::Json};

#[delete("/delete_flag/<id>")]
pub async fn delete_challenge_api(pool: &State<Pool>, token: JwtToken, id: String) -> status::Custom<Json<JsonResponse>> {
    let access = JwtToken::decode(token.body);

    match access {
        Ok(_) => {
            let query = remove_challenge(id, pool).await;
            match query{
                Ok(query) => {
                    let resp = JsonResponse {
                        id: String::from(query)
                    };
                    status::Custom(Status::Ok, Json(resp))
                },
                Err(_) => {
                    status::Custom(Status::NotFound, Json(JsonResponse { id: String::from("") }))
                }
            }
        },
        Err(_) => {
            status::Custom(Status::Forbidden, Json(JsonResponse { id: String::from("") }))
        }
    }
}
