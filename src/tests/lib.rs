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
        let accountid: u32 = 1940;
        let _query = sqlx::query!(
            r#"
            SELECT *
            FROM accounts
            WHERE accountid = ?;
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
            INSERT INTO accounts (accountid, username, firstname, lastname, password, origin, accesslevel)
            VALUES ("123","bigman", "Keanu","Reeves","dog","internal", "User");
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
        sqlx::query!(
            r#"
            INSERT INTO accounts (accountid, username, firstname, lastname, password, origin, accesslevel)
            VALUES ("123","bigman", "Keanu","Reeves","dog","internal", "User");
            "#)
            .execute(&pool)
            .await
            .unwrap();

        let _query = sqlx::query!(
            r#"
            SELECT *
            FROM accounts
            WHERE accountid = 123;
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
            INSERT INTO accounts (accountid, username, firstName, lastName, password, origin, accessLevel)
            VALUES ("122", "smallman", "Keanu","Reeves","dog","internal", "student");
            "#)
            .execute(&pool)
            .await
            .unwrap();

        let _query = sqlx::query!(
            r#"
            UPDATE accounts
            SET password = "goodbye"
            WHERE accountid = 123
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
            WHERE accountid = 123;
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
            .connect("mysql://zulu:zulu@localhost:3306/zulu")
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

}
