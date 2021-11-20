use actix_web::web;

use super::path::Path;

mod items;
mod content_loader;

pub fn app_factory(app: &mut web::ServiceConfig) {
    let base_bath = Path {
        prefix: String::from("/")
    };
    app.route(
        &base_bath.define(String::from("")),
        web::get().to(items::items)
    );
}