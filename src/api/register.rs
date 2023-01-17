use rocket::form::Form;
use rocket::State;
use super::super::auth::user::{User,AccessLevel};
use super::super::connections::database::Pool;

#[post("/register", data = "<user>")]
pub async fn register(pool: &State<Pool>, user: Form<User>){
    //Create a new user in the database
    let query = sqlx::query!(
        r#"
        INSERT INTO accounts (studentID, firstName, lastName, password, origin, accessLevel)
        VALUES (?,?,?,?,?,?);"#,
        &user.studentid,
        &user.firstname,
        &user.lastname,
        User::hash_password(&user.password),
        &user.origin,
        AccessLevel::User.to_string(),
        )
        .execute(&pool.0)
        .await;
    /*
     * Check if the user was entered correctly into the database, if any errors
     * occur handle them
     */
    match query {
        Ok(_query) => {
            info!("Registered new user: {}", &user.studentid); 
        },
        Err(_) =>{
            error!("Couldn't register user: {}", &user.studentid);
        } 
    }
}

