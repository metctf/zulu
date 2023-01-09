use rocket::form::Form;
use rocket::State;
use super::super::auth::user::User;
use super::super::auth::jwt::JwtToken;
use super::super::connections::database::Pool;

#[post("/modify", data = "<user>")]
pub async fn modify(pool: &State<Pool>, token: JwtToken, user: Form<User>) -> String{
    /* 
     * From the information pre occupied in the form fields this function
     * updates the database with any info thats changed.
     */
    let query = sqlx::query!(
        r#"
        UPDATE accounts
        SET studentID = ?,
        firstName = ?,
        lastName = ?,
        password = ?,
        origin = ?
        WHERE accountID = ?;
        "#,
        &user.studentid,
        &user.firstname,
        &user.lastname,
        User::hash_password(&user.password),
        &user.origin,
        &token.user_id
        )
        .execute(&pool.0)
        .await;

    /*
     * Redirect to a page or send a success code, undecided atm
     */
    match query{
        Ok(_query) => {format!("success")},
        Err(_) => {format!("boooo")}
    }

}

#[get("/")]
pub async fn get_user_info(pool: &State<Pool>, token: JwtToken) -> String{

    /*
     * Function that returns the user information to be displayed in the 
     * webpage to be edited by the user.
     */

    let query = sqlx::query!(
        r#"
        SELECT *
        FROM accounts
        WHERE accountID = ?;
        "#,
        &token.body
        )
        .fetch_one(&pool.0)
        .await;

    /*
     * Return info as json
     */
    match query {
        Ok(query) => format!("{:?}",query),
        Err(_) => format!("Unknown user")
    }
}
