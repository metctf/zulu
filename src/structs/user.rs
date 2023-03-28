use crate::auth::accesslevel::AccessLevel;
use crate::structs::origin::Origin;
use crate::structs::Verify;

use serde::Deserialize;
use serde::Serialize;
use argon2::Argon2;
use argon2::password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString};
use rand::rngs::OsRng;

#[derive(FromForm,Serialize,Deserialize,Debug)]
pub struct User{
    #[field(default = "")]
    pub accountid: String,
    pub username: String,
    pub firstname: String,
    pub lastname: String,
    pub password: String,
    pub origin: Origin,
    #[field(default = 0)]
    pub solves: u32,
    #[field(default = AccessLevel::User)]
    pub accesslevel: AccessLevel,
}

impl Verify for User {
    fn verify_password(&self, password: &str) -> bool{
        let argon2 = Argon2::default();
        let password_u8 = &self.password.as_bytes();
        let hash_parsed = PasswordHash::new(password).unwrap();
        let result = argon2.verify_password(password_u8, &hash_parsed).is_ok();
        result
    }
    fn verify_flag(&self, _flag: &str) -> bool {
        true
    }
}

impl User{
    pub fn hash_password(unhashed_password: &str) -> String{
        let argon2 = Argon2::default(); // init new instance of argon2 hashing
        let password_u8 = unhashed_password.as_bytes(); // get value of password as bytes
        let salt = SaltString::generate(&mut OsRng); // generate salt
        let hashed_password = argon2.hash_password(password_u8, &salt).unwrap().to_string(); // generate hash of password using salt
        hashed_password
    }
}

