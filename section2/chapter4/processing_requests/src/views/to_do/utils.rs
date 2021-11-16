use crate::json_serialization::to_do_items::ToDoItems;
use crate::state::read_file;
use crate::to_do::{ItemTypes, to_do_factory};

pub async fn return_state() {
    let state = read_file();

    let buffer = state.iter().map(|(key, value)| {
        let item_type = String::from(value.as_str().unwrap());
        to_do_factory(&item_type, key.as_str()).unwrap()
    }).collect::<Vec<ItemTypes>>();
    ToDoItems::new(buffer)
}