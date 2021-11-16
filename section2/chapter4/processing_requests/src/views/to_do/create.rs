use actix_web::HttpRequest;

use crate::to_do;
use crate::state::read_file;
use crate::processes::process_input;

pub async fn create(req: HttpRequest) -> String {
    let state = read_file();
    let title = req.match_info().get("title").unwrap().to_string();

    let title_reference = title.clone();
    let item = to_do::to_do_factory(&String::from("pending"), title.as_str()).expect("create");
    process_input(item, "create".to_string(), &state);
    format!("{} created", title_reference)
}