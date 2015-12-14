use openssl::crypto::hash::Type;
use openssl::crypto::hmac::hmac;
use openssl::crypto::hmac::HMAC;

use std::io::Write;

use rustc_serialize::hex::ToHex;

#[macro_use]
pub mod http;

pub fn hmac256(secret: &[u8], data: &[u8]) -> Vec<u8> {
    hmac(Type::SHA256, secret, data)
}

pub fn request_hmac(secret_str: &str, method: &str, path: &str, body: &str) -> String {
    // hmac(hmac(GET) + hmac(/ping) + hmac())
    let secret = secret_str.as_bytes();

    let method_hmac = hmac256(secret, method.as_bytes());
    let path_hmac = hmac256(secret, path.as_bytes());
    let body_hmac = hmac256(secret, body.as_bytes());

    let mut meta = HMAC::new(Type::SHA256, secret);
    meta.write_all(&method_hmac[..]).unwrap();
    meta.write_all(&path_hmac[..]).unwrap();
    meta.write_all(&body_hmac[..]).unwrap();

    meta.finish().to_hex()
}
