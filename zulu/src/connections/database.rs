use sqlx::mysql::MySqlPoolOptions;
use crate::auth::jwt::JwtToken;
use crate::structs::Verify;
use crate::structs::origin::Origin;

use sqlx::MySqlPool;

use rocket::State;
use rocket::form::Form;
use std::str::FromStr;

use uuid::Uuid;

use crate::auth::accesslevel::AccessLevel;
use crate::structs::user::User;
use crate::structs::login::Login;
use crate::structs::leaderboard::Leaderboard;
use crate::structs::challenge::{Challenge, SubmitChallenge};

pub async fn remove_challenge(id: String, pool: &State<Pool>) -> Result<String, sqlx::Error>{
    let result = sqlx::query!(
        r#"
        DELETE FROM challenges
        WHERE id = ?;
        "#,
        id)
        .fetch_all(&pool.0)
        .await;
    match result {
        Ok(_) => Ok(id),
        Err(_) => Err(sqlx::Error::RowNotFound)
    }
}

pub async fn create_challenge(challenge: &Form<Challenge>, pool: &State<Pool>) -> Result<String, sqlx::Error>{
    let uuid = Uuid::new_v4();
    let result = sqlx::query!(
        r#"
        INSERT INTO challenges
        (id, name, author, flag, points)
        VALUES
        (?,?,?,?,?)
        "#,
        format!("{}", uuid),
        &challenge.name,
        &challenge.author,
        Challenge::hash_flag(&challenge.flag),
        &challenge.points)
        .execute(&pool.0)
        .await;

    match result {
        Ok(_) => Ok(format!("{}", uuid)),
        Err(msg) => {
            error!("{}",msg);
            Err(sqlx::Error::PoolTimedOut)
        }
    }
}

pub async fn submit_challenge(sent_challenge: Form<SubmitChallenge>, pool: &State<Pool>) -> Result<bool, sqlx::Error>{
    let challenge_db = sqlx::query!(
        r#"
        SELECT *
        FROM challenges
        WHERE name = ?;
        "#,
        sent_challenge.name
        )
        .fetch_one(&pool.0)
        .await?;

    let challenge_str = Challenge {
        id: challenge_db.id,
        name: sent_challenge.name.clone(),
        author: challenge_db.author,
        flag: sent_challenge.flag.clone(),
        points: challenge_db.points,
    };

    if challenge_str.verify_flag(&challenge_db.flag) {
        Ok(true)
    }

    else {
        Ok(false)
    }
}

pub async fn modify_challenge(challenge: &Form<Challenge>, pool: &State<Pool>) -> Result<String, sqlx::Error>{
    let result = sqlx::query!(
        r#"
        UPDATE challenges
        SET name = ?,
        author = ?,
        flag = ?,
        points = ?
        WHERE
        id = ?;
        "#,
        &challenge.name,
        &challenge.author,
        &challenge.flag,
        &challenge.points,
        &challenge.id)
        .execute(&pool.0)
        .await;

    match result {
        Ok(_) => Ok(challenge.id.clone()),
        Err(_) => Err(sqlx::Error::RowNotFound)
    }
}

pub async fn return_challenge(pool: &State<Pool>, name: String) -> Result<Vec<Challenge>, sqlx::Error>{
    // send a flag to the database to retrieve the corresponding challenge
    let result = sqlx::query_as!(
        Challenge,
        "SELECT id, name, author, flag, points
        FROM challenges 
        WHERE name LIKE CONCAT('%',?,'%');",
        name)
        .fetch_all(&pool.0)
        .await?;
   Ok(result) 
}

pub async fn return_one_challenge(pool: &State<Pool>, name: String) -> Result<Challenge, sqlx::Error>{
    let result = sqlx::query_as!(
        Challenge,
        "SELECT id, name, author, flag, points
        FROM challenges 
        WHERE name = ?;",
        name)
        .fetch_one(&pool.0)
        .await?;
   Ok(result) 
}

pub async fn login_user(login: &Form<Login>, pool: &State<Pool>) -> Result<User,sqlx::Error>{
    let result = sqlx::query!(
        r#"
        SELECT *
        FROM accounts
        WHERE username = ?;"#,
        &login.username
        )
        .fetch_one(&pool.0)
        .await?;

    let pass = result.password;

    let user = User { 
        accountid: result.id, 
        username: result.username, 
        firstname: result.firstname, 
        lastname: result.lastname, 
        password: pass.unwrap(), 
        origin: Origin::from_str(&result.origin).unwrap(),
        solves: result.solves, 
        accesslevel: AccessLevel::from_str(&result.accesslevel).unwrap(),
    };
    info!("Logged in user: {}", &user.username);
    Ok(user)
}

pub async fn modify_user(user: &Form<User>, token: JwtToken, pool: &State<Pool>) -> Result<String,sqlx::Error>{
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
        WHERE id = ?;
        "#,
        &user.username,
        &user.firstname,
        &user.lastname,
        User::hash_password(&user.password),
        &user.origin.to_string(),
        &token.id
        )
        .execute(&pool.0)
        .await?;

    Ok(token.id)
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
        WHERE id = ?;
        "#,
        &token.body
        )
        .fetch_one(&pool.0)
        .await?;

    let pass = result.password;

    match pass {
        Some(pass) => {
            let user = User { 
                accountid: result.id, 
                username: result.username, 
                firstname: result.firstname, 
                lastname: result.lastname, 
                password: pass, 
                origin: Origin::from_str(&result.origin).unwrap(),
                solves: result.solves, 
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
        "SELECT username, solves FROM accounts
        ORDER BY solves DESC
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

pub async fn register_account(pool: &State<Pool>, user: &User) -> Result<String, sqlx::Error> {
    let uuid = Uuid::new_v4();
    //Create a new user in the database
    let _query = sqlx::query!(
        r#"
        INSERT INTO accounts (id, username, firstName, lastName, password, origin, accessLevel)
        VALUES (?,?,?,?,?,?,?);"#,
        format!("{}", uuid),
        &user.username,
        &user.firstname,
        &user.lastname,
        User::hash_password(&user.password),
        &user.origin.to_string(),
        AccessLevel::User.to_string(),
        )
        .execute(&pool.0)
        .await?;

    Ok(format!("{}", uuid))
}


pub async fn delete_account(pool: &State<Pool>, token: JwtToken) -> Result<bool, sqlx::Error> {
    let decoded = JwtToken::decode(token.body).unwrap();
    let query = sqlx::query!(
        r#"
        DELETE FROM accounts
        WHERE id = ?;"#,
        &decoded.id
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
