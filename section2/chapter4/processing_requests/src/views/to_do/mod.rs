use actix_web::web;

use crate::views::path::Path;

mod create;
mod get;
mod utils;

pub fn item_factory(app: &mut web::ServiceConfig) {
    let base_path = Path {
        prefix: "/item".to_string()
    };
    app.route(
        &base_path.define(String::from("/get")),
        web::get().to(get::get),
    ).route(
        &base_path.define(String::from("/create/{title}")),
        web::post().to(create::create),
    );
}
