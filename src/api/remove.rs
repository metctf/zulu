use rocket::State;
use super::super::auth::jwt::JwtToken;
use super::super::connections::database::Pool;

#[delete("/remove")]
pub async fn remove_account(pool: &State<Pool>, token: JwtToken){
    let decoded = JwtToken::decode(token.body).unwrap();
    let query = sqlx::query!(
        r#"
        DELETE FROM accounts
        WHERE accountID = ?;"#,
        &decoded.user_id
    )
    .execute(&pool.0)
    .await;

    match query {
        Ok(_query) => {
            info!("Removing account: {}", &decoded.user_id);
        },
        Err(_) => {
            error!("Cannot remove account: {}", &decoded.user_id); 
        }
    }
}
