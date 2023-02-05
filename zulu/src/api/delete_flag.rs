use rocket::State;
use crate::auth::jwt::JwtToken;
use super::super::connections::database::{Pool,remove_flag};

#[delete("/delete_flag/<id>")]
pub async fn delete_flag_api(pool: &State<Pool>, token: JwtToken, id: String) -> String{
    let access = JwtToken::decode(token.body);

    match access {
        Ok(_) => {
            let query = remove_flag(id, pool).await;
            match query{
                Ok(query) => {
                    if query{
                        return format!("Deleted flag")
                    }
                    return format!("Did not delete flag")
                },
                Err(_) => return format!("Bad Request")
            }
        },
        Err(_) => return format!("Unauthorized")
    }
}
