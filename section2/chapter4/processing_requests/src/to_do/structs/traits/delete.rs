use serde_json::{Map, Value};
use crate::state::write_to_file;

pub trait Delete {
    fn delete(&self, title: &str, state: &mut Map<String, Value>) {
        match state.remove(title) {
            Some(_) => {
                write_to_file(state);
                println!("{} has been deleted", title);
            }
            None => println!("{} not found", title)
        };
    }
}