use rocket::form::Form;
use rocket::time::{OffsetDateTime, Duration};
use rocket::http::{Cookie,CookieJar};
use rocket::State;
use crate::connections::database;

use rocket::http::Status;

use super::super::auth::jwt::JwtToken;
use crate::structs::login::Login;
use crate::structs::Verify;
use super::super::connections::database::Pool;
use super::super::connections::ldap;

#[post("/login", data = "<login>")]
pub async fn login(pool: &State<Pool>,jar: &CookieJar<'_>, login: Form<Login>) -> (Status, String) {
    info!("enter login");
    let user = database::login_user(&login, pool).await;
    match user {
        Ok(user) => {
            if login.verify_password(&user.password) {
                info!("Verified");
                // Add jwt token to a cookie to be returned to user
                let token = JwtToken::encode(&user.accountid.to_string(), &user.accesslevel.to_string());
                 // Create a new cookie with the authentication token
                let mut cookie = Cookie::new("Authentication", token);
                // Adds an expiration time of 1h
                let mut time = OffsetDateTime::now_utc();
                time += Duration::hours(12);
                cookie.set_expires(time);
                // Adds the cookie to the users browser
                jar.add_private(cookie);
                // Redirects to the sensitive page with Status 200
                return (Status::Ok, String::from("Successfully authenticated!"))
            }
            else {
                info!("Password invalid or no match in DB");
                let settings_result = crate::settings::LdapConfig::new();

                let settings = match settings_result {
                    Ok(settings) => settings,
                    Err(e) => return (Status::InternalServerError, String::from(format!("Error, {}", e))),
                };

                if user.accountid == 0 { // if no user was returned from the DB
                    let login_result = ldap::login_user(settings.hostname, settings.port, settings.bind_dn, settings.password, user.username, user.password, settings.search_base, settings.user_filter); // attempt to retrieve user from LDAP

                    let login = match login_result.await {
                        Ok(login) => login,
                        Err(e) => return (Status::InternalServerError, String::from(format!("Error, {}", e))), // Error message for server
                                                                       // error, returns 500
                    };

                    if login {
                        return (Status::Ok, String::from("Successfully authenticated!"))
                    }
                    else {
                        return (Status::Unauthorized, String::from("Unauthorised."))
                    }
                }

                else { // if user returned, but password was invalid
                    return (Status::Unauthorized, String::from("Unauthorised."))
                }
            }
        },
        // Error message for server error, returns 500
        Err(e) => return (Status::InternalServerError, String::from(format!("Error, {}", e)))
    }
}
