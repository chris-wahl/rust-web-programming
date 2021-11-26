use actix_web::{web, HttpResponse};
use diesel::prelude::*;

use crate::database::establish_connection;
use crate::diesel;
use crate::json_serialization::new_user::NewUserSchema;
use crate::models::user::new_user::NewUser;
use crate::schema::users;

pub async fn create(new_user: web::Json<NewUserSchema>) -> HttpResponse {
    let connection = establish_connection();

    let name = new_user.name.clone();
    let email = new_user.email.clone();
    let password = new_user.password.clone();

    let new_user = NewUser::new(name, email, password);

    let insert_reslt = diesel::insert_into(users::table)
        .values(&new_user)
        .execute(&connection);

    match insert_reslt {
        Ok(_) => HttpResponse::Created().await.unwrap(),
        Err(_) => HttpResponse::Conflict().await.unwrap(),
    }
}
