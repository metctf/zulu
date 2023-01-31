use sqlx::MySqlPool;
use sqlx::mysql::{MySqlPoolOptions, MySqlQueryResult};
use rocket::{Request, Response, State};
use rocket::fairing::{Fairing,Info,Kind};
use rocket::http::{Method, ContentType, Status};
use std::io::Cursor;
use crate::auth::jwt::JwtToken;

use rocket::form::Form;
use std::str::FromStr;

use crate::auth::user::{Login, User, AccessLevel, Leaderboard};

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
        WHERE username = ?;
        "#,
        &login.username
        )
        .fetch_one(&pool.0)
        .await?;

    let pass = result.password;

    match pass {
        Some(pass) => {
            let user = User { 
                accountid: result.accountid, 
                username: result.username, 
                firstname: result.firstname, 
                lastname: result.lastname, 
                password: pass, 
                origin: result.origin, 
                flagquantity: result.flagquantity, 
                accesslevel: AccessLevel::from_str(&result.accesslevel).unwrap(),
            };
            Ok(user)
        },
        None => Err(sqlx::Error::RowNotFound)
    }
}

pub async fn modify_user(user: &Form<User>, token: JwtToken, pool: &State<Pool>) -> Result<bool,sqlx::Error>{
    /* 
     * From the information pre occupied in the form fields this function
     * updates the database with any info thats changed.
     */
    sqlx::query!(
        r#"
        UPDATE accounts
        SET username = ?,
        firstName = ?,
        lastName = ?,
        password = ?,
        origin = ?
        WHERE accountID = ?;
        "#,
        &user.username,
        &user.firstname,
        &user.lastname,
        User::hash_password(&user.password),
        &user.origin,
        &token.accountid
        )
        .execute(&pool.0)
        .await?;

    Ok(true)
}

pub async fn get_user_info(token: JwtToken, pool: &State<Pool>) -> Result<User,sqlx::Error>{
    /*
     * Function that returns the user information to be displayed in the 
     * webpage to be edited by the user.
     */

    let result = sqlx::query!(
        r#"
        SELECT *
        FROM accounts
        WHERE accountID = ?;
        "#,
        &token.body
        )
        .fetch_one(&pool.0)
        .await?;

    let pass = result.password;

    match pass {
        Some(pass) => {
            let user = User { 
                accountid: result.accountid, 
                username: result.username, 
                firstname: result.firstname, 
                lastname: result.lastname, 
                password: pass, 
                origin: result.origin, 
                flagquantity: result.flagquantity, 
                accesslevel: AccessLevel::from_str(&result.accesslevel).unwrap(),
            };
            Ok(user)
        },
        None => Err(sqlx::Error::RowNotFound)
    }
}

pub async fn get_top_30(pool: &State<Pool>) -> Result<Vec<Leaderboard>,sqlx::Error> {
    let query = sqlx::query_as!(
        Leaderboard,
        "SELECT username, flagquantity FROM accounts
        ORDER BY flagquantity DESC
        LIMIT 30;")
        .fetch_all(&pool.0)
        .await?;
    Ok(query)
}

/* 
 * Empty pool struct to be managed by rocket, its been abstracted out 
 * of the api module so that module can be added to easier
 */
pub struct Pool(pub MySqlPool);

pub async fn create_connection() -> Result<MySqlPool, sqlx::Error> {
    let pool = MySqlPoolOptions::new()
        .max_connections(5)
        .connect("mysql://zulu:zulu@127.0.0.1:3306/zulu")
        .await?;
    Ok(pool)
}

pub async fn register_account(pool: &State<Pool>, user: &Form<User>) -> Result<MySqlQueryResult, sqlx::Error> {
    //Create a new user in the database
    let query = sqlx::query!(
        r#"
        INSERT INTO accounts (username, firstName, lastName, password, origin, accessLevel)
        VALUES (?,?,?,?,?,?);"#,
        &user.username,
        &user.firstname,
        &user.lastname,
        User::hash_password(&user.password),
        &user.origin,
        AccessLevel::User.to_string(),
        )
        .execute(&pool.0)
        .await?;

    Ok(query)
}


pub async fn delete_account(pool: &State<Pool>, token: JwtToken) -> Result<bool, sqlx::Error> {
    let decoded = JwtToken::decode(token.body).unwrap();
    let query = sqlx::query!(
        r#"
        DELETE FROM accounts
        WHERE accountID = ?;"#,
        &decoded.accountid
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
