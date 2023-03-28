use serde::Deserialize;
use serde::Serialize;
use crate::structs::Verify;
use argon2::password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString};
use argon2::Argon2;
use rand::rngs::OsRng;

#[derive(FromForm,Serialize,Deserialize,Debug,Default)]
pub struct Challenge {
    pub id: String,
    pub name: String,
    pub author: String,
    pub flag: String,
    pub points: u32,
}

#[derive(FromForm,Serialize,Deserialize)]
pub struct SubmitChallenge {
    pub name: String,
    pub flag: String,
}

impl Verify for Challenge {
    fn verify_password(&self, _password: &str) -> bool { // stub
        true
    }
    fn verify_flag(&self, flag: &str) -> bool{
        let argon2 = Argon2::default();
        let flag_u8 = &self.flag.as_bytes();
        info!("{}", flag);
        let hash_parsed = PasswordHash::new(flag).unwrap();
        let result = argon2.verify_password(flag_u8, &hash_parsed).is_ok();
        result
    }
}

impl Challenge {
    pub fn hash_flag(unhashed_flag: &str) -> String{
        let argon2 = Argon2::default(); // init new instance of argon2 hashing
        let flag_u8 = unhashed_flag.as_bytes(); // get value of password as bytes
        let salt = SaltString::generate(&mut OsRng); // generate salt
        let hashed_flag = argon2.hash_password(flag_u8, &salt).unwrap().to_string(); // generate hash of flag using salt
        hashed_flag
    }
}
