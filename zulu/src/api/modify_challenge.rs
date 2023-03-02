use rocket::form::Form;
use rocket::State;
use log::log;

use super::super::structs::challenge::Challenge;
use super::super::auth::jwt::JwtToken;
use super::super::connections::database::{Pool, modify_challenge, return_challenge};

use rocket::serde::{Serialize, json::Json};
use rocket::response::{content, status};
use rocket::http::Status;
use super::super::structs::json::JsonResponse;

#[post("/modify_challenge", data = "<challenge>")]
pub async fn modify_challenge_api(pool: &State<Pool>, token: JwtToken, challenge: Form<Challenge>) -> status::Custom<Json<JsonResponse>> {
    let access = JwtToken::decode(token.body);

    match access {
        Ok(_) => {
            let query = modify_challenge(&challenge, pool).await;
                match query{
                Ok(_query) => {
                    let resp = JsonResponse {
                        id: String::from(_query)
                    };
                    status::Custom(Status::Ok, Json(resp))
                }
                Err(_) => status::Custom(Status::NotFound, Json(JsonResponse { id: String::from("") }))
            }
        },
        Err(_) => status::Custom(Status::Forbidden, Json(JsonResponse { id: String::from("") }))

    }

}

#[get("/get_challenge/<flag>")]
pub async fn display_flag(pool: &State<Pool>, flag: String) -> status::Custom<Json<Vec<Challenge>>> {
    /*
     * Return info as json
     */
    let query = return_challenge(pool, flag).await;

    match query {
        Ok(query) => {
            status::Custom(Status::Ok, Json(query))
        }
        Err(query) => {
            error!("{:?}",query);
            let challenge = Challenge {
                id: String::from(""),
                name: String::from(""),
                author: String::from(""),
                flag: String::from(""),
                points: 0,
            };
            let mut vec: Vec<Challenge> = vec![];
            vec.push(challenge);

            status::Custom(Status::NotFound, Json(vec))
        }

    }
}
