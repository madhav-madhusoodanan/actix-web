use hmac::{Hmac, NewMac};
use sha2::Sha256;

pub fn key_function() -> Hmac<Sha256> {
    Hmac::new_from_slice(b"hehelolol").unwrap()
}


pub const url: &str = "mongodb+srv://madhav:madhav@cluster0.tlg4k.mongodb.net/";
pub enum BackendError {
    DbError,
}