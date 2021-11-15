use std::env;

use serde_json::json;

use processes::process_input;
use state::{read_file, write_to_file};

use crate::to_do::to_do_factory;

mod state;
mod to_do;
mod processes;

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut state = read_file();


    if args.len() > 2 {
        let command = &args[1];
        let title = &args[2];
        let status: String;
        match &state.get(*&title) {
            Some(result) => status = result.to_string().replace('\"', ""),
            None => status = "pending".to_string()
        }
        let item = to_do_factory(&status, title).expect(&status);
        process_input(item, command.to_string(), &state);
    }
}
