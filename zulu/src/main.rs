use env_logger::Builder;
use log::LevelFilter;
use crate::auth::rerouter::ReRouter;
use rocket::fs::FileServer;
use rocket::fs::relative;
mod connections;

mod settings;
use settings::LdapConfig;

mod api;
use api::{login,register,modify,remove,leaderboard,challenge};

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
               challenge::create_challenge_api,
               challenge::delete_challenge_api,
               challenge::modify_challenge_api,
               challenge::submit_challenge_api,
               challenge::display_flag,
               challenge::display_all_challenges,
               challenge::single_flag,
               challenge::author_challenge,
        ])
        .mount("/api/v1/static", FileServer::from(relative!("/static")))
        .attach(ReRouter)
        .attach(CORS)
}
