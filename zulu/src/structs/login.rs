use crate::structs::Verify;
use crate::structs::origin::Origin;

use serde::Deserialize;
use serde::Serialize;
use argon2::Argon2;
use argon2::password_hash::{PasswordHash, PasswordVerifier};


#[derive(FromForm,Serialize,Deserialize)]
pub struct Login{
    pub username: String,
    pub password: String,
    pub origin: Origin,
}

impl Verify for Login {
    fn verify_flag(&self, _flag: &str) -> bool { // stub
        true
    }
    fn verify_password(&self, password: &str) -> bool{
        let argon2 = Argon2::default();
        let password_u8 = &self.password.as_bytes();
        let hash_parsed = PasswordHash::new(password).unwrap();
        let result = argon2.verify_password(password_u8, &hash_parsed).is_ok();
        result
    }
}
