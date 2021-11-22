use actix_web::{web, HttpResponse};
use diesel::prelude::*;

use crate::database::establish_connection;
use crate::diesel;
use crate::json_serialization::to_do_item::ToDoItem;
use crate::schema::to_do;

use super::utils::return_state;

pub async fn edit(to_do_item: web::Json<ToDoItem>) -> HttpResponse {
    let title_ref = to_do_item.title.clone();
    let connection = establish_connection();
    let results = to_do::table.filter(to_do::columns::title.eq(title_ref));

    // Update the found items in `results` to a status of "done"
    let _ = diesel::update(results)
        .set(to_do::columns::status.eq("done"))
        .execute(&connection);

    return HttpResponse::Ok().json(return_state());
}
