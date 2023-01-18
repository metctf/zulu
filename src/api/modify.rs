use rocket::form::Form;
use rocket::State;

use super::super::auth::user::User;
use super::super::auth::jwt::JwtToken;
use super::super::connections::database::{Pool, modify_user, get_user_info};

#[post("/modify", data = "<user>")]
pub async fn modify(pool: &State<Pool>, token: JwtToken, user: Form<User>) -> String{
    let query = modify_user(&user, token, pool).await;
    /*
     * Redirect to a page or send a success code, undecided atm
     */
    match query{
        Ok(_query) => {format!("success")},
        Err(_) => {format!("boooo")}
    }

}

#[get("/")]
pub async fn display_user_info(pool: &State<Pool>, token: JwtToken) -> String{
    /*
     * Return info as json
     */
    let query = get_user_info(token, pool).await;

    match query {
        Ok(query) => format!("{:?}",query),
        Err(_) => format!("Unknown user")
    }
}
