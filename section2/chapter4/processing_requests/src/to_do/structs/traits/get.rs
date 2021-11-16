use serde_json::{Map, Value};

pub trait Get {
    fn get(&self, title: &str, state: &Map<String, Value>) {
        match state.get(title) {
            Some(result) => println!("\n\nItem: {}\n\tStatus: {}", title, result),
            None => println!("Item `{}` was not found", title)
        }
    }
}