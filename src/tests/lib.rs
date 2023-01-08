#[cfg(test)]
mod tests{

    /*
     * Test library for the major database tests
     */

    use sqlx::MySql;
    use sqlx::mysql::MySqlPoolOptions;

    #[sqlx::test]
    async fn login(){
        
    }

    #[sqlx::test]
    async fn register(){

    }

    #[sqlx::test]
    async fn get_user_info(){
 
    }

    #[sqlx::test]
    async fn modify (){

    }

    #[sqlx::test]
    async fn remove(){

    }

    #[sqlx::test]
    async fn create_connection_test(){
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
        let unhashed_password = String::from("test");
        let cost: u32 = 10;
        let hashed_password = bcrypt::hash(unhashed_password,cost).unwrap();
        println!("{}",hashed_password);
        let verify = bcrypt::verify("test", &hashed_password).unwrap();

        assert_eq!(verify,true);
    }

}
