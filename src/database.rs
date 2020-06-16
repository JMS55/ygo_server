use crate::Database;
use diesel::{ExpressionMethods, QueryDsl, Queryable, RunQueryDsl};
use scrypt::scrypt_check;

table! {
    users (id) {
        id -> Integer,
        username -> Text,
        password -> Text,
        is_admin -> Bool,
        duel_points -> Integer,
    }
}

#[derive(Queryable)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password: String,
    pub is_admin: bool,
    pub duel_points: i32,
}

pub fn authenticate_user_succeeded(username: &str, password: &str, db: &Database) -> bool {
    let users = users::table
        .filter(users::username.eq(&username))
        .load::<User>(&**db)
        .unwrap();
    users
        .iter()
        .any(|user| scrypt_check(password, &user.password).is_ok())
}
