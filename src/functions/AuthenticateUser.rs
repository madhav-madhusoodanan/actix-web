use hmac:: Hmac;
use jwt::VerifyWithKey;
use sha2::Sha265;
use std::collections::BTreeMap;

use crate::config::key;
pub fn AuthenticateUser(bearer: String) -> Result<String, String> {

    // extract the user from the token, given the secret key
    // and check if the user is available 
    let claim: BTreeMap<String, String> = bearer.verify_with_key(&key);
}