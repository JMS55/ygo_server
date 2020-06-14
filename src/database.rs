use crate::Database;
use diesel::{ExpressionMethods, Insertable, QueryDsl, Queryable, RunQueryDsl};
use scrypt::scrypt_check;

pub const DB_FALSE: i32 = 0;
pub const DB_TRUE: i32 = 1;

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

pub fn authenticate_user_succeeded(username: &str, password: &str, db: &Database) -> bool {
    let users = users::table
        .filter(users::username.eq(&username))
        .load::<User>(&**db)
        .unwrap();
    users
        .iter()
        .any(|user| user.username == username && scrypt_check(password, &user.password).is_ok())
}
