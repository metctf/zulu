#[cfg(test)]
mod tests{

    /*
     * Test library for the major database tests
     */
    
    use sqlx::MySqlPool;
    use sqlx::mysql::MySqlPoolOptions;

    #[sqlx::test]
    async fn login(pool: MySqlPool){
        // Test for checking database for user
        let accountid = "cd41294a-afb0-11df-bc9b-00241dd75637";
        let _query = sqlx::query!(
            r#"
            SELECT *
            FROM accounts
            WHERE accountID = ?;
            "#,
            accountid
            )
            .fetch_one(&pool)
            .await;

        match _query {
            Ok(_query) => assert!(true),
            Err(_query) => {
                panic!("Error with login function: {}", _query)
            }

        }
    }

    #[sqlx::test]
    async fn register(pool: MySqlPool){
        // Test for adding a user to a database
        let _query = sqlx::query!(
            r#"
            INSERT INTO accounts (username, firstName, lastName, password, origin, accessLevel)
            VALUES ("keanu.reeves","Keanu","Reeves","dog","internal", "User");
            "#)
            .execute(&pool)
            .await;

        match _query {
            Ok(_query) => assert!(true),
            Err(_query) => {
                panic!("Error with register function: {}", _query)
            }

        }
    }

    #[sqlx::test]
    async fn get_user_info(pool: MySqlPool){
        // Test for returning database info
        let _query = sqlx::query!(
            r#"
            SELECT *
            FROM accounts
            WHERE username = "winston.churchill";
            "#)
            .fetch_one(&pool)
            .await;

        match _query {
            Ok(_query) => assert!(true),
            Err(_query) => {
                panic!("Error getting user info: {}", _query)
            }

        }
    }

    #[sqlx::test]
    async fn modify (pool: MySqlPool){
        // Test for modifying user info
        sqlx::query!(
            r#"
            INSERT INTO accounts (username, firstName, lastName, password, origin, accessLevel)
            VALUES ("keanu.reeves","Keanu","Reeves","dog","Internal", "User");
            "#)
            .execute(&pool)
            .await
            .unwrap();

        let _query = sqlx::query!(
            r#"
            UPDATE accounts
            SET password = "goodbye"
            WHERE username = "keanu.reeves"
            AND password = "dog";
            "#)
            .execute(&pool)
            .await;

        match _query {
            Ok(_query) => assert!(true),
            Err(_query) => {
                panic!("Error modifying a user: {}", _query)
            }

        }
    }

    #[sqlx::test]
    async fn remove(pool: MySqlPool){
        // Test for removing users from a database
        let _query = sqlx::query!(
            r#"
            DELETE FROM accounts
            WHERE username = 123;
            "#)
            .execute(&pool)
            .await;

        match _query {
            Ok(_query) => assert!(true),
            Err(_query) => {
                panic!("Error removing a user: {}", _query)
            }

        }
    }

    #[sqlx::test]
    async fn create_connection_test(){
        // Test for checking if a database can be connected to
        let pool = MySqlPoolOptions::new()
            .max_connections(5)
            .connect("mysql://zulu:zulu@127.0.0.1:3306/zulu")
            .await;
        match pool{
            Ok(_pool) => assert!(true),
            Err(_pool) => assert!(false)
        }
    }

    /*
     * Tests for Jwt functions
     */

    use sha2::Sha256;
    use hmac::{Hmac, NewMac};
    use std::collections::BTreeMap;
    use jwt::{VerifyWithKey, SignWithKey};

    #[test]
    fn encode_and_decode_from_correct_token(){
        // Test for encoding and decoding a correct JwtToken
        let message: String = String::from("test");
        let key: Hmac<Sha256> = Hmac::new_varkey(&message.as_bytes()).unwrap();
        let mut claims = BTreeMap::new();
        claims.insert("sub","test");
        let token_str = claims.sign_with_key(&key).unwrap();

        let decode_claims: BTreeMap<String,String> = token_str.verify_with_key(&key).unwrap();
        assert_eq!(decode_claims["sub"],"test");

    }

    #[test]
    fn encode_and_decode_from_incorrect_token(){
        // Test for encoding and decoding an incorrect JwtToken
        let message: String =String::from("test");
        let key: Hmac<Sha256> = Hmac::new_varkey(&message.as_bytes()).unwrap();
        let mut claims = BTreeMap::new();
        claims.insert("user","test");
        let token_str = claims.sign_with_key(&key).unwrap();

        let decode_claims: BTreeMap<String,String> = token_str.verify_with_key(&key).unwrap();
        assert_ne!(decode_claims["user"],"incorrect token");
    }

    /*
     * Tests for Users and account level functions
     */

    #[test]
    fn hash_and_verify_password(){
        // Test for hashing and verifying a password
        let unhashed_password = String::from("test");
        let cost: u32 = 10;
        let hashed_password = bcrypt::hash(unhashed_password,cost).unwrap();
        println!("{}",hashed_password);
        let verify = bcrypt::verify("test", &hashed_password).unwrap();

        assert_eq!(verify,true);
    }

    /*
     * Test for LDAP API login functionality
     */

    /*use rocket::http::{ContentType, Status};
    use rocket::local::asynchronous::Client;

    #[rocket::async_test]
    async fn api_ldap_login(){
        let rocket = rocket::build();
        let client = Client::tracked(rocket).await.expect("valid rocket");
        let mut response = client.post("/api/v1/login")
            .body("username=jacob.eva&password=Edno8euzDE9BOdxENVbCZPhK5&origin=Ldap")
            .dispatch();
        assert_eq!(verify,true);
    }*/

    /*
     * Tests for loading config file
     */ 

    #[test]
    fn load_ldap_config() {
        let settings = crate::settings::LdapConfig::new();
        
        match settings {
            Ok(_) => assert!(true),
            Err(_) => assert!(false),
        }
    }


    /*
     * Tests for LDAP functions
     */

    #[tokio::test]
    async fn ldap_login() {
        let settings_result = crate::settings::LdapConfig::new();

        let settings = match settings_result {
            Ok(settings) => settings,
            Err(e) => panic!("Error, {}", e),
        };

        let login_result = crate::connections::ldap::login_user(settings.hostname, settings.port, settings.bind_dn, settings.password, String::from("jacob.eva"), String::from("password"), settings.search_base, settings.user_filter).await;

        let login = match login_result {
            Ok(login) => login,
            Err(e) => panic!("Error, {}", e),
        };
    }
}
