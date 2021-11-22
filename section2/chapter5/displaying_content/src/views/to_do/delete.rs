use actix_web::{web, HttpResponse};
use crate::json_serialization::to_do_item::ToDoItem;
use crate::processes::process_input;
use crate::state::read_file;
use crate::to_do::to_do_factory;
use crate::views::to_do::utils::return_state;

pub async fn delete(todo_item: web::Json<ToDoItem>) -> HttpResponse {
    let state = read_file("./state.json".to_string());
    let title = todo_item.title.clone();
    let status = todo_item.status.clone();

    match to_do_factory(&status, title) {
        Err(_) => return HttpResponse::BadRequest().json(format!("{} not accepted", status)),
        Ok(item) => process_input(item, String::from("delete"), &state)
    };
    return HttpResponse::Ok().json(return_state());
}