use diesel::{ExpressionMethods, PgConnection, QueryDsl, Queryable, RunQueryDsl};
use rocket_contrib::database;
use scrypt::scrypt_check;

#[database("database")]
pub struct Database(PgConnection);

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

impl Database {
    pub fn authenticate_user_succeeded(&self, username: &str, password: &str) -> bool {
        let users = users::table
            .filter(users::username.eq(&username))
            .load::<User>(&**self)
            .unwrap();
        users
            .iter()
            .any(|user| scrypt_check(password, &user.password).is_ok())
    }
}

table! {
    tournaments (id) {
        id -> Integer,
        name -> Text,
        rounds -> Integer,
        duel_points_per_win -> Integer,
        duel_points_per_loss -> Integer,
        duel_points_jackpot -> Integer,
    }
}

#[derive(Queryable)]
pub struct Tournament {
    pub id: i32,
    pub name: String,
    pub rounds: i32,
    pub duel_points_per_win: i32,
    pub duel_points_per_loss: i32,
    pub duel_points_jackpot: i32,
}

table! {
    matches (id) {
        id -> Integer,
        tournament -> Integer,
        round -> Integer,
        duelist1 -> Nullable<Integer>,
        duelist2 -> Nullable<Integer>,
        duelist1_reported_winning -> Nullable<Bool>,
        duelist2_reported_winning -> Nullable<Bool>,
    }
}

pub struct Match {
    pub id: i32,
    pub tournament: i32,
    pub round: i32,
    pub duelist1: Option<i32>,
    pub duelist2: Option<i32>,
    pub duelist1_reported_winning: Option<bool>,
    pub duelist2_reported_winning: Option<bool>,
}
