use ldap3::{LdapConnAsync, Scope, LdapError, SearchEntry};
use crate::structs::origin::Origin;
use crate::auth::accesslevel::AccessLevel;
use std::str::FromStr;

pub struct LdapUser {
    pub username: String,
    pub firstname: String,
    pub lastname: String,
    pub origin: Origin,
    pub accesslevel: AccessLevel
}

// This function checks if the user with the given UID can authenticate to the LDAP server, which
// if true means they have valid LDAP credentials.
pub async fn login_user(hostname: String, port: u16, bind_dn: String, bind_dn_password: String, uid: String, password: String, search_base: String, user_filter: String) -> Result<bool, LdapError> {
    let (conn, mut ldap) = LdapConnAsync::new(&*(String::from("ldap://") + &hostname + ":" + &port.to_string())).await?;
    ldap3::drive!(conn);
    ldap
        .simple_bind(&bind_dn, &bind_dn_password).await?
        .success()?;

    let (search, _res) = ldap.search(
        &search_base,
        Scope::Subtree,
        &user_filter.replace("%s", &uid),
        vec!["dn"]
    ).await?.success()?;

    if search.len() >= 1 {
        let entry = search[0].clone(); // have to clone the search entry else rust complains about
                                       // moving out of the var behind a shared reference
        ldap
            .simple_bind(&format!("{}", SearchEntry::construct(entry).dn), &password).await?
            .success()?;
        Ok(true)
    }
    else {
        Ok(false)
    }
}

// This function creates a User struct based on the attributes we can retrieve about the user from
// the LDAP server and returns it.
pub async fn retrieve_user(hostname: String, port: u16, bind_dn: String, bind_dn_password: String, uid: String, search_base: String, lecturer_filter: String, admin_filter: String) -> Result<LdapUser, LdapError> {
    let role;
    let (conn, mut ldap) = LdapConnAsync::new(&*(String::from("ldap://") + &hostname + ":" + &port.to_string())).await?;
    ldap3::drive!(conn);
    ldap
        .simple_bind(&bind_dn, &bind_dn_password).await?
        .success()?;

    // Checks if the user is an admin
    let (search, _res) = ldap.search(
        &search_base,
        Scope::Subtree,
        &admin_filter.replace("%s", &uid),
        vec!["uid"]
    ).await?.success()?;

    if search.len() > 0 {
        role = String::from("Admin");
    }

    else {
        // Checks if the user is a lecturer
        let (search, _res) = ldap.search(
            &search_base,
            Scope::Subtree,
            &lecturer_filter.replace("%s", &uid),
            vec!["uid"]
        ).await?.success()?;

        if search.len() > 0 {
            role = String::from("Lecturer");
        }

        else {
            role = String::from("User");
        }
    }

    let split: Vec<&str> = uid.split(".").collect(); // split the uid in two, as the first part is
                                                     // the forename, and last part is the surname

    let user = LdapUser { 
        username: uid.clone(),  // clone as uid is used earlier in the split
        firstname: split[0].to_string(), 
        lastname: split[1].to_string(), 
        origin: Origin::from_str("CMET").unwrap(), // CHANGEME: Actually ascertain which university
                                                   // the user is from
        accesslevel: AccessLevel::from_str(&role).unwrap(),
    };
    Ok(user)
}
