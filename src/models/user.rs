use diesel::prelude::*;

#[derive(Queryable)]
pub struct User {
    id: i32,
    discord_id: String,
}