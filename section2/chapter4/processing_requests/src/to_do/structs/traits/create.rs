use serde_json::{json, Map, Value};
use crate::state::write_to_file;

pub trait Create {
    fn create(&self, title: &String, status: &String, state: &mut Map<String, Value>) {
        state.insert(title.to_string(), json!(status));
        write_to_file(state);
        println!("{} has been created", title);
    }
}