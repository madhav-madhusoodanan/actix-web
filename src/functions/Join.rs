use hmac::{Hmac, NewMac};
use jwt::SignWithKey;
use sha2::Sha256;
use std::collections::BTreeMap;
use argon2::{
    password_hash::{
        rand_core::OsRng,
        PasswordHash, PasswordHasher, PasswordVerifier, SaltString
    },
    Argon2
};

use crate::config::key;

pub fn Login(username: &String, password: &String) -> Result<String, str> {
    // here we gotta implement access tokens

    // PROPERTIES of bcrypt: 
    // must be able to successfully compare password and salted hash
    
    // PROPERTIES of jwt: 
    // given a secret token and the username, we gotta give him or her
    // access token. this access token will be verified.

    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let hash = argon2.hash_password(password.as_bytes(), &salt)?;
    if argon2.verify_password(password, &parsed_hash).is_ok() {
        makeAccessToken(username)
    } else Err("wrong password or username")
}

pub fn Signup(username: &String, password: &String) -> Result<String, String> {
    // first make a salted hash of the password and store in database
    // create the user
    // make an accessToken

    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let hash = argon2.hash_password(password.as_bytes(), &salt)?.to_string();

    // make a user object and send it to database
    
    makeAccessToken(username)
}

fn makeAccessToken(username: &String) -> Result<String, String> {
    let mut claims = BTreeMap::new();
    claims.insert("user", username);

    let token = claims.sign_with_key(&key)?;
    Ok(token)
}