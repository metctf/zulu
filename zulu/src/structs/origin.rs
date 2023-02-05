use serde::Deserialize;
use serde::Serialize;
use rocket::form::FromFormField;
use std::fmt;
use std::str::FromStr;

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
