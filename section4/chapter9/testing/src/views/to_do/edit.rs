use actix_web::{HttpRequest, HttpResponse, web};
use diesel::prelude::*;

use crate::auth::jwt::JwtToken;
use crate::database::establish_connection;
use crate::diesel;
use crate::json_serialization::to_do_item::ToDoItem;
use crate::schema::to_do;

use super::utils::return_state;

pub async fn edit(to_do_item: web::Json<ToDoItem>, req: HttpRequest) -> HttpResponse {
    let title_ref = to_do_item.title.clone();
    let connection = establish_connection();
    let token = JwtToken::decode_from_request(req).unwrap();
    let results = to_do::table.filter(to_do::columns::title.eq(title_ref))
        .filter(to_do::columns::user_id.eq(&token.user_id));

    // Update the found items in `results` to a status of "done"
    let _ = diesel::update(results)
        .set(to_do::columns::status.eq("done"))
        .execute(&connection);

    return HttpResponse::Ok().json(return_state(&token.user_id));
}
