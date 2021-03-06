use actix_web::dev::ServiceRequest;

fn check_password(password: String) -> Result<String, &'static str> {
    if password == "token" { Ok(password) } else { Err("token not authorized") }
}

fn extract_header_token(request: &ServiceRequest) -> Result<String, &'static str> {
    match request.headers().get("user-token") {
        Some(token) => {
            match token.to_str() {
                Ok(processed_password) => Ok(String::from(processed_password)),
                Err(_) => Err("there was an error processing the token")
            }
        }
        None => Err("token missing")
    }
}

pub fn process_token(request: &ServiceRequest) -> Result<String, &'static str> {
    match extract_header_token(request) {
        Ok(token) => check_password(token),
        Err(message) => Err(message)
    }
}