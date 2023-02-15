use rocket::form::Form;
use rocket::time::{OffsetDateTime, Duration};
use rocket::http::{Cookie,CookieJar};
use rocket::State;
use crate::auth::accesslevel::AccessLevel;
use crate::structs::json::JsonJwtResponse;

use rocket::serde::{Serialize, json::Json};
use rocket::response::{content, status};
use rocket::http::Status;

use super::super::connections::database;

use crate::structs::origin::Origin; // for creating origin of new user from LDAP

use std::str::FromStr; // for getting origin and accesslevel from string

use super::super::auth::jwt::JwtToken;
use crate::structs::login::Login;
use crate::structs::Verify;
use super::super::connections::ldap;
use crate::structs::user::User;
use super::super::connections::database::{Pool, register_account};

#[post("/login", data = "<login>")]
pub async fn login(pool: &State<Pool>,jar: &CookieJar<'_>, login: Form<Login>) -> status::Custom<Json<JsonJwtResponse>> {
    info!("enter login");
    let user = database::login_user(&login, pool).await;
    match user {
        Ok(user) => {
            if login.verify_password(&user.password) {
                info!("Verified");
                // Add jwt token to a cookie to be returned to user
                let token = JwtToken::encode(&user.accountid.to_string(), &user.accesslevel.to_string());

                let resp = JsonJwtResponse {
                    jwt: token
                };

                // Redirects to the sensitive page with Status 200
                return status::Custom(Status::Ok, Json(resp))
            }
            else { // if user returned, but password was invalid
                return status::Custom(Status::Forbidden, Json(JsonJwtResponse { jwt: String::from("") }))
            }
        },
        Err(e) => {
            // TODO: make this section trigger only if the Origin::Ldap is in the login form
            if format!("{:?}", e) == "RowNotFound" { // check if error type is row not found,
                                                     // meaning no match from the DB
                info!("RowNotFound triggered, testing LDAP login");
                let settings_result = crate::settings::LdapConfig::new();

                let settings = match settings_result {
                    Ok(settings) => settings,
                    Err(_) => return status::Custom(Status::InternalServerError, Json(JsonJwtResponse { jwt: String::from("") })),
                };

                let login_valid_result = ldap::login_user(settings.hostname.clone(), settings.port, settings.bind_dn.clone(), settings.password.clone(), login.username.clone(), login.password.clone(), settings.search_base.clone(), settings.user_filter.clone()); // attempt to retrieve user from LDAP and login as them

                let login_valid = match login_valid_result.await {
                    Ok(login) => login,
                    Err(_e) => { 
                        error!("{}", _e);
                        return status::Custom(Status::InternalServerError, Json(JsonJwtResponse { jwt: String::from("") }))
                    }
                                                                       // error, returns 500
                };

                if login_valid {
                    let ldapuser_struct_result = ldap::retrieve_user(settings.hostname, settings.port, settings.bind_dn, settings.password, login.username.clone(), settings.search_base, settings.lecturer_filter, settings.admin_filter).await;


                    let ldapuser_struct = match ldapuser_struct_result {
                        Ok(user) => user,
                        Err(_) => return status::Custom(Status::InternalServerError, Json(JsonJwtResponse { jwt: String::from("") })),
                    };

                    let user_struct = User {
                        accountid: String::from(""),
                        username: ldapuser_struct.username,
                        firstname: ldapuser_struct.firstname,
                        lastname: ldapuser_struct.lastname,
                        password: login.password.clone(),
                        origin: ldapuser_struct.origin,
                        flagquantity: 0,
                        accesslevel: ldapuser_struct.accesslevel,
                    };

                    let uuid = register_account(pool, &user_struct).await; // register the user
                                                                           // from LDAP
                    match uuid {
                        Ok(uuid) => {
                            // Add jwt token to a cookie to be returned to user
                            let token = JwtToken::encode(&format!("{}", uuid), &user_struct.accesslevel.to_string());

                            let resp = JsonJwtResponse {
                                jwt: token
                            };

                            // Redirects to the sensitive page with Status 200
                            info!("Registered new user from LDAP: {}", &user_struct.username); 
                            return status::Custom(Status::Ok, Json(resp))
                        },
                        Err(_) =>{
                            return status::Custom(Status::InternalServerError, Json(JsonJwtResponse { jwt: String::from("") }))
                        } 
                    }
                }
                else {
                    return status::Custom(Status::Ok, Json(JsonJwtResponse { jwt: String::from("") }))
                }
            }
            else {
                return status::Custom(Status::InternalServerError, Json(JsonJwtResponse { jwt: String::from("") }))
            }
        }
    }
}
