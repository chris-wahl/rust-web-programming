use std::env;

use serde_json::{json, Map};
use serde_json::value::Value;

use state::{read_file, write_to_file};
use to_do::{ItemTypes, structs::traits::create::Create, to_do_factory};

mod state;
mod to_do;

const FILEPATH: &str = "/home/christopher/Documents/Rust/Rust Web Programming/chapter2/chapter2-todo/src/state.json";

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut state = read_file(FILEPATH);
    println!["{:?}", state];

    if args.len() > 2 {
        let status = &args[1];
        let title = &args[2];

        state.insert(title.to_string(), json!(status));
        write_to_file(FILEPATH, &mut state);
    }
}
