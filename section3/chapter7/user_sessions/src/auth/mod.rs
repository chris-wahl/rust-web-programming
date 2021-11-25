use actix_web::dev::ServiceRequest;

mod processes;
pub(crate) mod jwt;

pub fn process_token(request: &ServiceRequest) -> Result<String, &'static str> {
    match processes::extract_header_token(request) {
        Ok(token) => processes::check_password(token),
        Err(message) => Err(message),
    }
}
