use rocket::form::Form;
use rocket::time::{OffsetDateTime, Duration};
use rocket::http::{Cookie,CookieJar};
use rocket::State;
use crate::connections::database;

use super::super::auth::jwt::JwtToken;
use super::super::auth::user::{Login,Verify};
use super::super::connections::database::Pool;
use super::super::connections::ldap;

#[post("/login", data = "<login>")]
pub async fn login(pool: &State<Pool>,jar: &CookieJar<'_>, login: Form<Login>) -> Result<(),std::string::String>{
    info!("enter login");
    let user = database::login_user(&login, pool).await;
    match user {
        Ok(user) => {
            if login.verify_password(&user.password){
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
                Ok(())
            }else{
                info!("else!");
                // Deny access to website, returns a 401 Error
                Ok(())
            }
        },
        // Error message for server error, returns 500
        Err(_) => {
            info!("Error from DB");
            let settings_result = crate::settings::LdapConfig::new();

            let settings = match settings_result {
                Ok(settings) => settings,
                Err(e) => return Err(format!("Error, {}", e)),
            };

            let login_result = ldap::login_user(settings.hostname, settings.port, settings.bind_dn, String::from("jacob.eva"), settings.password, settings.search_base, settings.user_filter);

            let login = match login_result.await {
                Ok(login) => login,
                Err(e) => return Err(format!("Error, {}", e)),
            };
            Ok(())
        }
    }
}
