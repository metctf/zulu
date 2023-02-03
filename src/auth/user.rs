use serde::Deserialize;
use serde::Serialize;
use rocket::form::FromFormField;
use std::fmt;
use std::str::FromStr;
use argon2::Argon2;
use argon2::password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString};
use rand::rngs::OsRng;

/*
 * File containing structs for all the forms and implementations for those
 * structs
 */
#[derive(FromFormField,Serialize,Deserialize,Debug)]
pub enum AccessLevel{
    Admin,
    Lecturer,
    User
}

impl fmt::Display for AccessLevel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
       match *self {
           AccessLevel::Admin => write!(f,"Admin"),
           AccessLevel::Lecturer => write!(f,"Lecturer"),
           AccessLevel::User => write!(f,"User"),
       }
    }
}

impl FromStr for AccessLevel {

    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Admin"  => Ok(AccessLevel::Admin),
            "Lecturer"  => Ok(AccessLevel::Lecturer),
            "User"  => Ok(AccessLevel::User),
            _      => Err(()),
        }
    }
}
#[derive(FromForm,Serialize,Deserialize,Debug)]
pub struct User{
    pub accountid: String,
    pub username: String,
    pub firstname: String,
    pub lastname: String,
    pub password: String,
    pub origin: String,
    pub flagquantity: u32,
    #[field(default = AccessLevel::User)]
    pub accesslevel: AccessLevel,
}

#[derive(Debug,Serialize,Deserialize)]
pub struct Leaderboard{
    pub username: String,
    pub flagquantity: u32,
}

#[derive(FromFormField,Serialize,Deserialize,Debug)]
pub enum Origin{
    Internal,
    Ldap,
}

impl fmt::Display for Origin {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
       match *self {
           Origin::Internal => write!(f,"Internal"),
           Origin::Ldap => write!(f,"Ldap"),
       }
    }
}

impl FromStr for Origin {

    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Internal"  => Ok(Origin::Internal),
            "Ldap"  => Ok(Origin::Ldap),
            _      => Err(()),
        }
    }
}

#[derive(FromForm,Serialize,Deserialize)]
pub struct Login{
    pub username: String,
    pub password: String,
    pub origin: Origin,
}

pub trait Verify {
    fn verify_password(&self, password: &str) -> bool;
}

impl Verify for Login {
    fn verify_password(&self, password: &str) -> bool{
        let argon2 = Argon2::default();
        let password_u8 = &self.password.as_bytes();
        let hash_parsed = PasswordHash::new(password).unwrap();
        let result = argon2.verify_password(password_u8, &hash_parsed).is_ok();
        result
    }
}

impl Verify for User {
    fn verify_password(&self, password: &str) -> bool{
        let argon2 = Argon2::default();
        let password_u8 = &self.password.as_bytes();
        let hash_parsed = PasswordHash::new(password).unwrap();
        let result = argon2.verify_password(password_u8, &hash_parsed).is_ok();
        result
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

/*
 * Making a default user that has the lowest access level and default
 * credentials
 *
 * The access levels for the user in decending order are:
 * 1. Admin
 * 2. Lecturer
 * 3. User
 *
 */
