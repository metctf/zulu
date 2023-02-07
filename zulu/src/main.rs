use env_logger::Builder;
use log::LevelFilter;
use crate::auth::rerouter::ReRouter;
mod connections;
use connections::ldap;

mod settings;
use settings::LdapConfig;

mod api;
use api::{login,register,modify,remove,leaderboard,create_flag,modify_flag,delete_flag,submit_flag};

mod auth;
mod tests;
mod structs;

use auth::cors::CORS;

#[macro_use] extern crate rocket;

#[launch]
async fn rocket() -> _ {
    //Builder for good looking logs
    let mut builder = Builder::from_default_env();
    builder
        .filter(None, LevelFilter::Info)
        .init();
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
               remove::remove_account,
               leaderboard::leaderboard,
               create_flag::create_flag_api,
               delete_flag::delete_flag_api,
               modify_flag::modify_flag_api,
               modify_flag::display_flag,
               submit_flag::submit_flag_api
        ])
        .attach(ReRouter)
        .attach(CORS)
        
}
