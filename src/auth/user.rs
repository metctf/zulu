use rocket::time::OffsetDateTime;
use serde::Deserialize;
use serde::Serialize;
use rocket::form::FromFormField;
use std::fmt;
use std::str::FromStr;

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
    pub accountid: u32,
    pub username: String,
    pub firstname: String,
    pub lastname: String,
    pub password: String,
    pub origin: String,
    pub flagquantity: u32,
    pub accesslevel: AccessLevel,
}

#[derive(Debug,Serialize,Deserialize)]
pub struct Leaderboard{
    pub username: String,
    pub flagquantity: u32,
}

#[derive(FromForm,Serialize,Deserialize)]
pub struct Login{
    pub username: String,
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
 * 2. Lecturer
 * 3. User
 *
 */
