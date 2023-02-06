use rocket::State;

use crate::connections::database::{Pool, submit_flag};

#[get("/submit_flag/<id>")]
pub async fn submit_flag_api(pool: &State<Pool>, id: String) -> String {
    let query = submit_flag(id, pool).await;
    match query {
        Ok(_) => format!("Correct"),
        Err(_) => format!("Incorrect")
    }
}
