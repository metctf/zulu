use config::{Config, ConfigError, File};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct LdapConfig {
    pub hostname: String, // used to connect to LDAP server
    pub port: u16,
    pub bind_dn: String, // user to connect as
    pub password: String, // bindDN password
    pub search_base: String, // the search base to use for getting users
    pub user_filter: String, // the filter used to identify users (e.g. posixAccount)
    pub lecturer_filter: String, // the filter used to identify lecturers (e.g. memberOf)
    pub admin_filter: String 
}

impl LdapConfig {
    pub fn new() -> Result<Self, ConfigError> {
        let config = Config::builder()
            // Start off by merging in the "default" configuration file
            .add_source(File::with_name("config/default"))
            // Add in a local configuration file
            // This file shouldn't be checked in to git
            .add_source(File::with_name("config/local").required(false))
            .build()?;

        // You can deserialize (and thus freeze) the entire configuration as
        config.try_deserialize()
    }
}
