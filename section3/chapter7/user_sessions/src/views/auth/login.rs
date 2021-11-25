use crate::diesel;
use diesel::prelude::*;
use actix_web::{web, HttpResponse};

use crate::database::establish_connection;
use crate::models::user::user::User;
use crate::json_serialization::login::Login;
use crate::schema::users;
use crate::auth::process_token;

pub async fn login() -> String {
    format!("Login view")
}
