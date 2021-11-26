use actix_web::web;

use super::path::Path;

mod content_loader;
mod items;
mod login;
mod logout;

pub fn app_factory(app: &mut web::ServiceConfig) {
    let base_bath = Path {
        prefix: String::from("/"),
        backend: false,
    };
    app.route(
        &base_bath.define(String::from("")),
        web::get().to(items::items),
    );
    app.route(
        &base_bath.define(String::from("login")),
        web::get().to(login::login),
    );

    app.route(
        &base_bath.define(String::from("logout")),
        web::get().to(logout::logout),
    );
}
