extern crate hmac;
extern crate jwt;
extern crate sha2;

use std::collections::BTreeMap;

use actix_web::HttpRequest;
use hmac::{Hmac, NewMac};
use jwt::{Header, SignWithKey, Token, VerifyWithKey};
use sha2::Sha256;

pub struct JwtToken {
    pub user_id: i32,
    pub body: String,
}

impl JwtToken {
    pub fn encode(user_id: i32) -> String {
        let key: Hmac<Sha256> = Hmac::new_varkey(b"secret").unwrap();
        let mut claims = BTreeMap::new();
        claims.insert("user_id", user_id);
        let token_str = claims.sign_with_key(&key).unwrap();
        return token_str;
    }

    pub fn decode(encoded_token: String) -> Result<JwtToken, &'static str> {
        let key: Hmac<Sha256> = Hmac::new_varkey(b"secret").unwrap();
        let token_str = encoded_token.as_str();
        let token: Result<Token<Header, BTreeMap<String, i32>, _>, _> = VerifyWithKey::verify_with_key(token_str, &key);

        match token {
            Ok(token) => {
                let _header = token.header();
                let claims = token.claims();
                return Ok(JwtToken {
                    user_id: claims["user_id"],
                    body: encoded_token,
                });
            }
            Err(_) => return Err("Could not decode")
        }
    }

    pub fn decode_from_request(request: HttpRequest) -> Result<JwtToken, &'static str> {
        match request.headers().get("user-token") {
            Some(token) => JwtToken::decode(token.to_str().unwrap().to_string()),
            None => Err("there is no token")
        }
    }
}