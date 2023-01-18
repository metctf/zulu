mod connections;
use connections::{ldap, database::ReRouter};

mod settings;
use settings::LdapConfig;

mod api;
use api::{login,register,modify,remove};

mod auth;
mod tests;

#[macro_use] extern crate rocket;

#[launch]
async fn rocket() -> _ {
    // Starts a Database connection
    let pool = connections::database::create_connection().await.unwrap();

    rocket::build()
        .manage(LdapConfig::new())
        .manage(connections::database::Pool(pool))
        .mount("/api/v1",routes![
               login::login,
               register::register,
               modify::modify,
               modify::display_user_info,
               remove::remove_account
        ])
        .attach(ReRouter)
        
}
