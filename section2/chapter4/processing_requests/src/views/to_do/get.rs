use actix_web::Responder;

use crate::json_serialization::to_do_items::ToDoItems;
use crate::state::read_file;
use crate::to_do::{ItemTypes, to_do_factory};
use crate::views::to_do::utils::return_state;

pub async fn get() -> impl Responder {
    return_state()
}