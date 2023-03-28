use rocket::data::{Data, ToByteUnit};
use std::io::Error;
use std::path::PathBuf;
use std::fs;

pub async fn create_challenge_file(name: &String, path: &PathBuf) -> Result<bool, Error> {
    // todo, add max file limit
    fs::copy(path, format!("static/challenge/{}", name))?;
    Ok(true)
}
