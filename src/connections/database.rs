use sqlx::MySqlPool;
use sqlx::mysql::MySqlPoolOptions;
use rocket::{Request, Response, State};
use rocket::fairing::{Fairing,Info,Kind};
use rocket::http::{Method, ContentType, Status};
use std::io::Cursor;
<<<<<<< HEAD
use crate::auth::jwt::JwtToken;
=======
use rocket::State;
use rocket::form::Form;

use crate::auth::user::{Login, User, AccessLevel, FromStr};
>>>>>>> 052d83f5180b9f51e67e2316b51e4275bd0414bb

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

pub async fn login_user(login: &Form<Login>, pool: &State<Pool>) -> Result<User,sqlx::Error>{
    let result = sqlx::query!(
        r#"
        SELECT *
        FROM accounts
        WHERE studentID = ?;
        "#,
        &login.studentid
        )
        .fetch_one(&pool.0)
        .await?;
    
    let user = User { 
        accountid: result.accountID, 
        studentid: result.studentID, 
        firstname: result.firstName, 
        lastname: result.lastName, 
        password: result.password, 
        origin: result.origin, 
        flagquantity: result.flagQuantity.unwrap(), 
        accesslevel: AccessLevel::from_str(result.accessLevel)
    };
    Ok(user)
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
