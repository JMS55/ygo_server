use diesel::{Insertable, Queryable};

table! {
    users (username) {
        username -> Text,
        password -> Text,
        is_admin -> Integer,
        duel_points -> Integer,
    }
}

#[derive(Queryable, Insertable)]
pub struct User {
    pub username: String,
    pub password: String,
    pub is_admin: i32,
    pub duel_points: i32,
}
