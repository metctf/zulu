use rocket::form::Form;
use rocket::State;

use super::super::structs::flag::Flag;
use super::super::auth::jwt::JwtToken;
use super::super::connections::database::{Pool, modify_flag, return_flag};

#[post("/modify_flag", data = "<flag>")]
pub async fn modify_flag_api(pool: &State<Pool>, token: JwtToken, flag: Form<Flag>) -> String{
    let access = JwtToken::decode(token.body);

    match access {
        Ok(_) => {
            let query = modify_flag(&flag, pool).await;
                match query{
                Ok(_query) => format!("success"),
                Err(_) => format!("boooo")
            }
        },
        Err(_) => format!("Unauthorized")
    }

}

#[get("/get_flag/<id>")]
pub async fn display_flag(pool: &State<Pool>, id: String) -> String{
    /*
     * Return info as json
     */
    let query = return_flag(pool, id).await;

    match query {
        Ok(query) => format!("{:?}",query),
        Err(_) => format!("Unknown flag")
    }
}
