use rocket::form::Form;
use rocket::time::{OffsetDateTime, Duration};
use rocket::http::{Cookie,CookieJar};
use rocket::State;
use super::super::auth::jwt::JwtToken;
use super::super::auth::user::{Login,Verify};
use super::super::connections::database::Pool;

#[post("/login", data = "<login>")]
pub async fn login(pool: &State<Pool>,jar: &CookieJar<'_>, login: Form<Login>) -> Result<(),()>{
    let user = sqlx::query!(
        r#"
        SELECT *
        FROM accounts
        WHERE studentID = ?;
        "#,
        &login.studentid
        )
        .fetch_one(&pool.0)
        .await;
    match user {
        Ok(user) => {
            // I really dont like having an unwrap here
            if login.verify_password(&user.password.unwrap()){
                // Add jwt token to a cookie to be returned to user
                let token = JwtToken::encode(&user.accountID.to_string(), &user.accessLevel.to_string());
                 // Create a new cookie with the authentication token
                let mut cookie = Cookie::new("Authentication", token);
                // Adds an expiration time of 1h
                let mut time = OffsetDateTime::now_utc();
                time += Duration::hours(12);
                cookie.set_expires(time);
                // Adds the cookie to the users browser
                jar.add_private(cookie);
                // Redirects to the sensitive page with Status 200
                Ok(())
            }else{
                // Deny access to website, returns a 401 Error
                Ok(())
            }
        },
        Err(_) => Err(()) 
            // Error message for server error, returns 500
    }
}
