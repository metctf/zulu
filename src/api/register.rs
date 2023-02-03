use rocket::form::Form;
use rocket::State;
use super::super::auth::user::User;
use super::super::connections::database::{Pool, register_account};

#[post("/register", data = "<user>")]
pub async fn register(pool: &State<Pool>, user: Form<User>){
    let query = register_account(pool, &user).await;
    /*
     * Check if the user was entered correctly into the database, if any errors
     * occur handle them
     */
    match query {
        Ok(_query) => {
            info!("Registered new user: {}", &user.username); 
        },
        Err(_) =>{
            error!("Couldn't register user: {}", &user.username);
        } 
    }
}

