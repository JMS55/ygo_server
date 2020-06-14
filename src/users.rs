use crate::database::{users, User};
use crate::{AdminKey, Database};
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
use rocket::{get, post, State};
use rocket_contrib::json;
use rocket_contrib::json::{Json, JsonValue};
use scrypt::{scrypt_simple, ScryptParams};
use serde::Deserialize;

#[get("/list")]
pub fn list(db: Database) -> JsonValue {
    let users = users::table.load::<User>(&*db).unwrap();
    let user_list = users
        .into_iter()
        .map(|user| {
            json!({
                "username": user.username,
                "is_admin": user.is_admin != 0,
            })
        })
        .collect::<Vec<JsonValue>>();
    json!({
        "r.type": "response",
        "user_list": user_list,
    })
}

#[derive(Deserialize)]
pub struct AddRequest {
    username: String,
    password: String,
    admin_key: Option<String>,
}

#[post("/add", data = "<request>")]
pub fn add(db: Database, state_admin_key: State<AdminKey>, request: Json<AddRequest>) -> JsonValue {
    let AddRequest {
        admin_key,
        username,
        mut password,
        ..
    } = request.into_inner();

    let username_not_taken = users::table
        .filter(users::username.eq(&username))
        .load::<User>(&*db)
        .unwrap()
        .is_empty();
    if username_not_taken {
        password = scrypt_simple(&password, &ScryptParams::recommended()).unwrap();
        if let Some(admin_key) = admin_key {
            if admin_key == state_admin_key.0 {
                diesel::insert_into(users::table)
                    .values(&User {
                        username,
                        password,
                        is_admin: 1,
                        duel_points: 0,
                    })
                    .execute(&*db)
                    .unwrap();
                json!({
                    "r.type": "response",
                })
            } else {
                json!({
                    "r.type": "error",
                    "info": "Invalid admin key.",
                })
            }
        } else {
            diesel::insert_into(users::table)
                .values(&User {
                    username,
                    password,
                    is_admin: 0,
                    duel_points: 0,
                })
                .execute(&*db)
                .unwrap();
            json!({
                "r.type": "response",
            })
        }
    } else {
        json!({
            "r.type": "error",
            "info": "User already exists.",
        })
    }
}

#[derive(Deserialize)]
pub struct DeleteRequest {
    username: String,
    password: String,
}

#[post("/delete", data = "<request>")]
pub fn delete(db: Database, request: Json<DeleteRequest>) -> JsonValue {
    let password = scrypt_simple(&request.password, &ScryptParams::recommended()).unwrap();
    let authentication_succeeded = !users::table
        .filter(users::username.eq(&request.username))
        .filter(users::password.eq(&password))
        .load::<User>(&*db)
        .unwrap()
        .is_empty();
    if authentication_succeeded {
        diesel::delete(users::table)
            .filter(users::username.eq(&request.username))
            .filter(users::password.eq(&password))
            .execute(&*db)
            .unwrap();
        json!({
            "r.type": "response",
        })
    } else {
        json!({
            "r.type": "error",
            "info": "Invalid username/password.",
        })
    }
}
