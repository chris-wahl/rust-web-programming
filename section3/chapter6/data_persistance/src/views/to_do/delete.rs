use actix_web::{HttpResponse, web};
use diesel::prelude::*;

use crate::database::establish_connection;
use crate::diesel;
use crate::json_serialization::to_do_item::ToDoItem;
use crate::models::item::item::Item;
use crate::schema::to_do;
use crate::views::to_do::utils::return_state;

pub async fn delete(todo_item: web::Json<ToDoItem>) -> HttpResponse {
    let title_ref = todo_item.title.clone();
    let connection = establish_connection();
    let items = to_do::table.filter(
        to_do::columns::title.eq(title_ref)
    )
        .order(to_do::columns::id.asc())
        .load::<Item>(&connection)
        .unwrap();
    let _ = diesel::delete(&items[0]).execute(&connection);
    return HttpResponse::Ok().json(return_state());
}