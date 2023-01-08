use rocket::form::Form;
use rocket::State;
use rocket::http::Status;
use super::super::auth::jwt::JwtToken;
use super::super::auth::user::{Login,Verify};
use super::super::connections::database::Pool;
use super::response::ApiResponse;

#[post("/login", data = "<login>")]
pub async fn login(pool: &State<Pool>, login: Form<Login>) -> ApiResponse{
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
                // Add jwt token to header to be returned to user
                let token = JwtToken::encode(&user.accountID.to_string());
                // Form a response and send a 200 Ok status
                ApiResponse {
                    auth_token: token.to_string(),
                    status: Status::Ok
                }
            }else{
                // Deny access to website, returns a 401 Error
                ApiResponse {
                    auth_token: format!("Null"),
                    status: Status::Unauthorized
                }
            }
        },
        Err(_) => 
            // Error message for server error, returns 500
            ApiResponse {
                auth_token: format!("Null"),
                status: Status::InternalServerError
            }
    }
}
