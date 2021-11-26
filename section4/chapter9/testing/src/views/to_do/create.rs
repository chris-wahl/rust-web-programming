use actix_web::{HttpRequest, Responder};
use diesel::prelude::*;

use crate::auth::jwt::JwtToken;
use crate::database::establish_connection;
use crate::diesel;
use crate::models::item::{item::Item, new_item::NewItem};
use crate::schema::to_do;
use crate::views::to_do::utils::return_state;

/// This view creates a to do item and saves it in the state.json file.
///
/// # Arguments
/// * req (HttpRequest): the HTTP request passed into the view
///
/// # Returns
/// * (String): message to be sent back to the user
pub async fn create(req: HttpRequest) -> impl Responder {
    let title = req.match_info().get("title").unwrap().to_string();
    let title_ref = title.clone();
    let connection = establish_connection();
    let token = JwtToken::decode_from_request(req).unwrap();

    let items = to_do::table
        // Find any items with the same title
        .filter(to_do::columns::title.eq(title_ref.as_str()))
        .filter(to_do::columns::user_id.eq(&token.user_id))
        .order(to_do::columns::id.asc())
        .load::<Item>(&connection)
        .unwrap();

    if items.len() == 0 {
        // No matching items found.  Create one.
        let new_post = NewItem::new(title, token.user_id);
        let _ = diesel::insert_into(to_do::table)
            .values(&new_post)
            .execute(&connection);
    }

    return return_state(&token.user_id);
}
