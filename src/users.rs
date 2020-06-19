use crate::database::{users, Database, User};
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
use rocket::{get, post};
use rocket_contrib::json;
use rocket_contrib::json::{Json, JsonValue};
use scrypt::{scrypt_simple, ScryptParams};
use serde::Deserialize;

#[get("/list")]
pub fn list(db: Database) -> JsonValue {
    let users = users::table.load::<User>(&*db).unwrap();
    let user_list = users
        .into_iter()
        .map(|user| user.username)
        .collect::<Vec<String>>();
    json!({
        "r.type": "response",
        "user_list": user_list,
    })
}

#[derive(Deserialize)]
pub struct ProfileRequest {
    username: String,
    password: String,
}

#[post("/profile", data = "<request>")]
pub fn profile(db: Database, request: Json<ProfileRequest>) -> JsonValue {
    if db
        .authenticate_user_succeeded(&request.username, &request.password)
        .is_ok()
    {
        let user = users::table
            .filter(users::username.eq(&request.username))
            .first::<User>(&*db)
            .unwrap();
        // TODO: Show cards
        json!({
            "r.type": "response",
            "duel_points": user.duel_points,
        })
    } else {
        json!({
            "r.type": "error",
            "info": "Invalid username/password.",
        })
    }
}

#[derive(Deserialize)]
pub struct AddRequest {
    username: String,
    password: String,
}

#[post("/add", data = "<request>")]
pub fn add(db: Database, mut request: Json<AddRequest>) -> JsonValue {
    let username_not_taken = users::table
        .filter(users::username.eq(&request.username))
        .load::<User>(&*db)
        .unwrap()
        .is_empty();
    if username_not_taken {
        request.password = scrypt_simple(&request.password, &ScryptParams::recommended()).unwrap();
        diesel::insert_into(users::table)
            .values((
                users::username.eq(&request.username),
                users::password.eq(&request.password),
                users::duel_points.eq(0),
            ))
            .execute(&*db)
            .unwrap();
        json!({
            "r.type": "response",
        })
    } else {
        json!({
            "r.type": "error",
            "info": "User already exists.",
        })
    }
}
