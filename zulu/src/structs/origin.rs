use serde::Deserialize;
use serde::Serialize;
use rocket::form::FromFormField;
use std::fmt;
use std::str::FromStr;

#[derive(FromFormField,Serialize,Deserialize,Debug)]
pub enum Origin{
    CMET,
}

impl fmt::Display for Origin {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
       match *self {
           Origin::CMET => write!(f,"CMET"),
       }
    }
}

impl FromStr for Origin {

    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "CMET"  => Ok(Origin::CMET),
            _      => Err(()),
        }
    }
}
