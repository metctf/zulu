use serde::Deserialize;
use serde::Serialize;

/*
 * File containing structs for all the forms and implementations for those structs
 */

#[derive(FromForm,Serialize,Deserialize)]
pub struct User{
    pub accountid: u32,
    pub studentid: String,
    pub firstname: String,
    pub lastname: String,
    pub password: String,
    pub origin: String,
    pub flagquantity: u32,
}

#[derive(FromForm,Serialize,Deserialize)]
pub struct Login{
    pub studentid: String,
    pub password: String,
}

pub trait Verify {
    fn verify_password(&self, password: &str) -> bool;
}

impl Verify for Login {
    fn verify_password(&self, password: &str) -> bool{
        bcrypt::verify(&self.password, &password).unwrap()
    }
}

impl Verify for User {
    fn verify_password(&self, password: &str) -> bool{
        bcrypt::verify(&self.password, &password).unwrap()
    }
}

impl User{
    pub fn hash_password(unhashed_password: &str) -> String{
        let cost: u32 = 10;
        let hashed_password = bcrypt::hash(unhashed_password,cost).unwrap();
        hashed_password
    }
}
