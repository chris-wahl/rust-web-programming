use crate::schema::to_do;
use super::super::user::user::User;

#[derive(Queryable, Identifiable)]
#[belongs_to(User)]
#[table_name = "to_do"]
pub struct Item {
    pub id: i32,
    pub title: String,
    pub status: String,
    pub user_id: i32
}