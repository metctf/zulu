use ldap3::{LdapConnAsync, Scope, SearchEntry, LdapError, Ldap};
use rocket::State;

#[tokio::main]
pub async fn get_accounts(hostname: String, port: u16, bind_dn: String, password: String, search_base: String, user_filter: String) -> Result<String, LdapError> {
    let (conn, mut ldap) = LdapConnAsync::new(&*(String::from("ldap://") + &*hostname + ":" + &*port.to_string())).await?;
    ldap3::drive!(conn);
    ldap
        .simple_bind(&*bind_dn, &*password).await?
        .success()?;

    let (search, _res) = ldap.search(
        &*search_base,
        Scope::Subtree,
        &*user_filter,
        vec!["uid"]
    ).await?.success()?;
    let mut users = String::new();
    for entry in search {
        users += &*format!("{:?}", SearchEntry::construct(entry.clone()));
        println!("{:?}", SearchEntry::construct(entry));
    }
    println!("exiting get_accounts");
    Ok(users)
}
