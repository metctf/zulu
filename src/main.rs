mod connections;
use connections::ldap;
mod settings;
use settings::LdapConfig;

#[macro_use] extern crate rocket;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .manage(LdapConfig::new())
}
