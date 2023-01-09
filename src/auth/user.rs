use serde::Deserialize;
use serde::Serialize;

/*
 * File containing structs for all the forms and implementations for those
 * structs
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
    pub accesslevel: String,
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

/*
 * Making a default user that has the lowest access level and default
 * credentials
 *
 * The access levels for the user in decending order are:
 * 1. Admin
 * 2. User
 * 3. Guest
 *
 */

impl Default for User {
    fn default() -> User {
        User { 
            accountid: 0,
            studentid: format!("Default"),
            firstname: format!("Default"),
            lastname: format!("Default"),
            password: format!("Default"),
            origin: format!("Default"),
            flagquantity: 0,
            accesslevel: format!("User")
        }
    }
}


