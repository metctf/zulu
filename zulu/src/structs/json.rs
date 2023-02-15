use rocket::response::{self, Response, Responder};
use rocket::http::{ Status};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct JsonResponse {
    pub id: String,
}


#[derive(Serialize, Deserialize)]
pub struct JsonJwtResponse {
    pub jwt: String,
}
