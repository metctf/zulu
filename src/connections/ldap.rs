use ldap3::{LdapConnAsync, Scope, SearchEntry, LdapError, Ldap, ResultEntry};
use crate::auth::user::{Login, User, AccessLevel, Leaderboard};
use log::{info, trace, warn};

// This function checks if the user with the given UID can authenticate to the LDAP server, which
// if true means they have valid LDAP credentials.
pub async fn login_user(hostname: String, port: u16, bind_dn: String, uid: String, password: String, search_base: String, user_filter: String) -> Result<Vec<ResultEntry>, LdapError> {
    let (conn, mut ldap) = LdapConnAsync::new(&*(String::from("ldap://") + &hostname + ":" + &port.to_string())).await?;
    ldap3::drive!(conn);
    ldap
        .simple_bind(&bind_dn, &password).await?
        .success()?;

    let (search, _res) = ldap.search(
        &search_base,
        Scope::Subtree,
        &user_filter.replace("%s", &uid),
        vec!["uid"]
    ).await?.success()?;

    Ok(search)
}

// This function creates a User struct based on the attributes we can retrieve about the user from
// the LDAP server and returns it.
/*pub async fn retrieve_user(hostname: String, port: u16, bind_dn: String, uid: String, password: String, search_base: String, user_filter: String, lecturer_filter: String, admin_filter: String) -> Result<String, LdapError> {
    let (conn, mut ldap) = LdapConnAsync::new(&*(String::from("ldap://") + &hostname + ":" + &port.to_string())).await?;
    ldap3::drive!(conn);
    ldap
        .simple_bind(&bind_dn, &password).await?
        .success()?;

    // Checks if the user is an admin
    let (search, _res) = ldap.search(
        &search_base,
        Scope::Subtree,
        &admin_filter.replace("%s", &uid),
        vec!["uid"]
    ).await?.success()?;

    if search.len() > 0 {
        let role = String::from("Admin");
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
            let role = String::from("Lecturer");
        }

        else {
            let role = String::from("User");
        }
    }

    let user = User { 
        accountid: result.accountid, 
        username: result.username, 
        firstname: result.firstname, 
        lastname: result.lastname, 
        password: pass, 
        origin: result.origin, 
        flagquantity: result.flagquantity, 
            let user = User { 
                accountid: result.accountid, 
                username: result.username, 
                firstname: result.firstname, 
                lastname: result.lastname, 
                password: pass, 
                origin: result.origin, 
                flagquantity: result.flagquantity, 
                accesslevel: AccessLevel::from_str(&result.accesslevel).unwrap(),
            };
            Ok(user)
        accesslevel: AccessLevel::from_str(&result.accesslevel).unwrap(),
    };
    Ok(user)
}
*/
