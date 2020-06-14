use crate::database::{authenticate_user_succeeded, users, User, DB_FALSE, DB_TRUE};
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
                "is_admin": user.is_admin == DB_TRUE,
            })
        })
        .collect::<Vec<JsonValue>>();
    json!({
        "r.type": "response",
        "user_list": user_list,
    })
}

#[derive(Deserialize)]
pub struct ViewRequest {
    username_to_view: String,
    username: String,
    password: String,
}

#[post("/view", data = "<request>")]
pub fn view(db: Database, request: Json<ViewRequest>) -> JsonValue {
    if authenticate_user_succeeded(&request.username, &request.password, &db) {
        let user = users::table
            .filter(users::username.eq(&request.username_to_view))
            .load::<User>(&*db)
            .unwrap();
        if let Some(user) = user.get(0) {
            if request.username_to_view == request.username {
                json!({
                    "r.type": "response",
                    "is_admin": user.is_admin == DB_TRUE,
                    "duel_points": user.duel_points,
                })
            } else {
                json!({
                    "r.type": "response",
                    "is_admin": user.is_admin == DB_TRUE,
                })
            }
        } else {
            json!({
                "r.type": "error",
                "info": "User does not exist.",
            })
        }
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
                        is_admin: DB_TRUE,
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
                    is_admin: DB_FALSE,
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
    if authenticate_user_succeeded(&request.username, &request.password, &db) {
        diesel::delete(users::table)
            .filter(users::username.eq(&request.username))
            .filter(users::password.eq(&request.password))
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
