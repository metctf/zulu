use rocket::State;
use super::super::auth::jwt::JwtToken;
use super::super::connections::database::{Pool,delete_account};

#[delete("/remove")]
pub async fn remove_account(pool: &State<Pool>, token: JwtToken) -> String{
    let query = delete_account(pool, token).await;
    match query{
        Ok(query) => {
            if query{
                return format!("Deleted account")
            }
            return format!("Did not delete account")
        },
        Err(_) => return format!("Bad Request")
    }
}
