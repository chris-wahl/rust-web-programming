use to_do::{ItemTypes, to_do_factory, structs::traits::create::Create};

mod to_do;


fn main() {
    let todo_item = to_do_factory("pending", "washing");
    match todo_item.unwrap() {
        ItemTypes::Pending(item) => item.create(&item.super_struct.title),
        ItemTypes::Done(item) => println!("Pending item: {}", item.super_struct.title),
    }
}
