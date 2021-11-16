use std::fs;
use std::fs::File;
use std::io::Read;

use serde_json::Map;
use serde_json::value::Value;
use serde_json::json;

const FILEPATH: &str = "./state.json";

pub fn read_file() -> Map<String, Value> {
    let mut file = File::open(FILEPATH.to_string()).expect("File not found!");
    let mut data = String::new();

    file.read_to_string(&mut data).unwrap();
    let json: Value = serde_json::from_str(&data).unwrap();
    let state: Map<String, Value> = json.as_object().unwrap().clone();
    return state;
}

pub fn write_to_file(state: &mut Map<String, Value>) {
    let new_data = json!(state);
    fs::write(FILEPATH.to_string(), new_data.to_string()).expect("Unable to write file!");
}