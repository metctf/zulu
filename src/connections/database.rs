use sqlx::MySqlPool;
use sqlx::mysql::MySqlPoolOptions;
use rocket::{Request, Response, State};
use rocket::fairing::{Fairing,Info,Kind};
use rocket::http::{Method, ContentType, Status};
use std::io::Cursor;
use crate::auth::jwt::JwtToken;

pub struct ReRouter;

#[rocket::async_trait]
impl Fairing for ReRouter {
    
    fn info(&self) -> Info {
        Info {
            name: "GET rerouter",
            kind: Kind::Response
        }
    }

    async fn on_response<'r>(&self, request: &'r Request<'_>,response: &mut Response<'r>) {
        if request.method() == Method::Get &&
            response.status() == Status::NotFound {
                let body = format!("URL does not exist");
                response.set_status(Status::Ok);
                response.set_header(ContentType::Plain);
                response.set_sized_body(body.len(),Cursor::new(body));
            }
        return
    }
}

/* 
 * Empty pool struct to be managed by rocket, its been abstracted out 
 * of the api module so that module can be added to easier
 */
pub struct Pool(pub MySqlPool);

pub async fn create_connection() -> Result<MySqlPool, sqlx::Error> {
    let pool = MySqlPoolOptions::new()
        .max_connections(5)
        .connect("mysql://zulu:zulu@localhost:3306/zulu")
        .await?;
    Ok(pool)
}

pub async fn delete_account(pool: &State<Pool>, token: JwtToken) -> Result<bool, sqlx::Error> {
    let decoded = JwtToken::decode(token.body).unwrap();
    let query = sqlx::query!(
        r#"
        DELETE FROM accounts
        WHERE accountID = ?;"#,
        &decoded.user_id
    )
    .execute(&pool.0)
    .await?
    .rows_affected();

    if query >= 1 {
        Ok(true)
    }
    
    else {
        Ok(false)
    }
}
