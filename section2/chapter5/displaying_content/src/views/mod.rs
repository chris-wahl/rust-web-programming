use actix_web::web;

mod path;
mod auth;
mod to_do;
pub mod token;
mod app;


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
}
