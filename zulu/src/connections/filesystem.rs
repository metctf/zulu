use rocket::data::{Data, ToByteUnit};
use std::io::Error;

pub async fn create_challenge_file(name: String, file: Data<'_>) -> Result<bool, Error> {
    file.open(100.megabytes()).into_file(format!("static/challenge/{}", name)).await?;
    Ok(true)
}
