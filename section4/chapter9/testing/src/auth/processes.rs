use actix_web::dev::ServiceRequest;

use super::jwt;

pub fn check_password(password: String) -> Result<String, &'static str> {
    match jwt::JwtToken::decode(password) {
        Ok(_token) => Ok(String::from("passed")),
        Err(message) => Err(message)
    }
}

pub fn extract_header_token(request: &ServiceRequest) -> Result<String, &'static str> {
    match request.headers().get("user-token") {
        Some(token) => match token.to_str() {
            Ok(processed_password) => Ok(String::from(processed_password)),
            Err(_) => Err("there was an error processing the token"),
        },
        None => Err("token missing"),
    }
}

#[cfg(test)]
mod check_credentials_tests {
    use std::ptr::null;

    use actix_web::test;

    use super::check_password;
    use super::extract_header_token;
    use super::super::jwt::JwtToken;

    #[test]
    fn correct_check_password() {
        let encoded_token = JwtToken::encode(-1);
        match check_password(encoded_token) {
            Ok(msg) => assert_eq!("passed", msg),
            _ => panic!("token should have decoded properly")
        };
    }

    #[test]
    fn incorrect_check_password() {
        let failing_token = String::from("a bad token");
        match check_password(failing_token) {
            Err(msg) => assert_eq!("Could not decode", msg),
            _ => panic!["Token should have failed to decode"]
        }
    }

    #[test]
    fn no_token_in_extract_header_token() {
        let mock_request = test::TestRequest::with_header("test", "test").to_srv_request();
        match extract_header_token(&mock_request) {
            Err(msg) => assert_eq!(msg, "token missing"),
            _ => panic!["token missing should fail"]
        }
    }

    #[test]
    fn correct_token_in_extract_header_token() {
        let token_value = "test";
        let mock_request = test::TestRequest::with_header("user-token", token_value.to_string()).to_srv_request();
        match extract_header_token(&mock_request) {
            Ok(msg) => assert_eq!(msg, token_value),
            _ => panic!["present token should have decoded"]
        }
    }
}