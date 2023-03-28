use serde::Deserialize;
use serde::Serialize;
use rocket::form::FromFormField;
use std::fmt;
use std::str::FromStr;


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
