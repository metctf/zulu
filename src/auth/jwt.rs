use jwt::{Header, Token, VerifyWithKey, SignWithKey};
use sha2::Sha256;
use std::collections::BTreeMap;
use std::result::Result;
use hmac::{Hmac, NewMac};
use rocket::http::Status;
use rocket::request::{self, Request, FromRequest};
use rocket::outcome::Outcome;

#[derive(Debug)]
pub struct JwtToken {
    pub id: String,
    pub accesslevel: String,
    pub body: String,
}

#[derive(Debug)]
pub enum ApiKeyError{
    Invalid,
    Missing,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for JwtToken{
    type Error = ApiKeyError;

    async fn from_request(req: &'r Request<'_>) -> request::Outcome<Self,Self::Error> {
        // Pulls all the headers from the request
        let header = req.headers().get_one("auth");
        // Decodes the token stored in the Authorization header
        match header {
            Some(header) => {
                let decoded = JwtToken::decode(header.to_string());
                // Checks the token for Authorization
                match decoded {
                    Ok(decoded) => Outcome::Success(decoded),
                    Err(_) => Outcome::Failure((Status::Unauthorized, ApiKeyError::Invalid))
                    }
                },
            // If the Header is wrong the user is forbidden from accessing it
            None => Outcome::Failure((Status::Forbidden, ApiKeyError::Missing))
        }
    }
}

impl JwtToken {
    //Encodes the JWT using a hashmap and attaches a SHA256 hash 
    pub fn encode(accountid: &String, accesslevel: &String) -> String{
        let secret_key: String = String::from("secret");
        let key: Hmac<Sha256> = Hmac::new_varkey(&secret_key.as_bytes())
            .unwrap();
        /*
         *  The private claims for the jwt should be as follows
         *
         *  {
         *      "accountid": accountid,
         *      "accesslevel": accesslevel,
         *  }
         *
         *  These claims allow for user identification as well as access 
         *  control
         */

        let mut claims = BTreeMap::new();
        claims.insert("id", accountid);
        claims.insert("accesslevel", accesslevel);

        let token_str = claims.sign_with_key(&key).unwrap();
        return String::from(token_str)
    }

    //Decodes the token to see if its valid
    pub fn decode(webtoken: String) -> Result<JwtToken, &'static str>{
        let secret_key: String = String::from("secret");
        let key: Hmac<Sha256> = Hmac::new_varkey(&secret_key.as_bytes()).unwrap();
        let token_str: &str = webtoken.as_str();

        let token: Result<Token<Header, BTreeMap<String, String>, _ > ,jwt::Error> = 
            token_str.verify_with_key(&key);

        match token {
            Ok(token) => Ok( JwtToken {
                id: token.claims()["id"].to_string(),
                accesslevel: token.claims()["accesslevel"].to_string(),
                body: webtoken}),
            Err(_) => Err("Couldn't Decode token")
            }
        }
    }
