use hmac:: Hmac;
use jwt::SignWithKey;
use sha2::Sha265;
use std::collections::BTreeMap;

let secret = b"hehelolol";
pub const key: Hmac<Sha256> = Hmac::new_from_slice(secret).unwrap();