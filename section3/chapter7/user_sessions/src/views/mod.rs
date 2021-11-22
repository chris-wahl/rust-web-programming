use actix_web::web;

mod app;
mod auth;
mod path;
mod to_do;
pub mod token;
mod users;

/// This function combines the views from other view modules.
///
/// # Arguments
/// * (&mut web::ServiceConfig): reference to the app for configuration
///
/// # Returns
/// None
pub fn views_factory(app: &mut web::ServiceConfig) {
    app::app_factory(app);
    auth::auth_factory(app);
    to_do::item_factory(app);
    users::user_factory(app);
}
