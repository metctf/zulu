use rocket::form::Form;
use rocket::State;
use log::log;

use super::super::structs::flag::Flag;
use super::super::auth::jwt::JwtToken;
use super::super::connections::database::{Pool, modify_flag, return_flag};

use rocket::serde::{Serialize, json::Json};
use rocket::response::{content, status};
use rocket::http::Status;
use super::super::structs::json::JsonResponse;

#[post("/modify_flag", data = "<flag>")]
pub async fn modify_flag_api(pool: &State<Pool>, token: JwtToken, flag: Form<Flag>) -> status::Custom<Json<JsonResponse>> {
    let access = JwtToken::decode(token.body);

    match access {
        Ok(_) => {
            let query = modify_flag(&flag, pool).await;
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

#[get("/get_flag/<challenge>")]
pub async fn display_flag(pool: &State<Pool>, challenge: String) -> status::Custom<Json<Flag>> {
    /*
     * Return info as json
     */
    let query = return_flag(pool, challenge).await;

    match query {
        Ok(_query) => {
            status::Custom(Status::Ok, Json(_query))
        }
        Err(query) => {
            error!("{:?}",query);
            let flag = Flag {
                flagid: String::from(""),
                challenge: String::from(""),
                challengeauthor: String::from(""),
                flagstring: String::from(""),
                points: 0,
            };
            status::Custom(Status::NotFound, Json(flag))
        }

    }
}
