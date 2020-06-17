use crate::database::{users as users_db, Database, User};
use crate::AdminKey;
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
use rocket::{get, post, State};
use rocket_contrib::json;
use rocket_contrib::json::{Json, JsonValue};
use scrypt::{scrypt_simple, ScryptParams};
use serde::Deserialize;

#[get("/list")]
pub fn list(db: Database) -> JsonValue {
    let users = users_db::table.load::<User>(&*db).unwrap();
    let user_list = users
        .into_iter()
        .map(|user| {
            json!({
                "username": user.username,
                "is_admin": user.is_admin,
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
    if db.authenticate_user_succeeded(&request.username, &request.password) {
        let user = users_db::table
            .filter(users_db::username.eq(&request.username_to_view))
            .load::<User>(&*db)
            .unwrap();
        if let Some(user) = user.get(0) {
            if request.username_to_view == request.username {
                // TODO: Show cards
                json!({
                    "r.type": "response",
                    "is_admin": user.is_admin,
                    "duel_points": user.duel_points,
                })
            } else {
                json!({
                    "r.type": "response",
                    "is_admin": user.is_admin,
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
pub fn add(
    db: Database,
    state_admin_key: State<AdminKey>,
    mut request: Json<AddRequest>,
) -> JsonValue {
    let username_not_taken = users_db::table
        .filter(users_db::username.eq(&request.username))
        .load::<User>(&*db)
        .unwrap()
        .is_empty();
    if username_not_taken {
        request.password = scrypt_simple(&request.password, &ScryptParams::recommended()).unwrap();
        if let Some(admin_key) = &request.admin_key {
            if admin_key == &state_admin_key.0 {
                diesel::insert_into(users_db::table)
                    .values((
                        users_db::username.eq(&request.username),
                        users_db::password.eq(&request.password),
                        users_db::is_admin.eq(true),
                        users_db::duel_points.eq(0),
                    ))
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
            diesel::insert_into(users_db::table)
                .values((
                    users_db::username.eq(&request.username),
                    users_db::password.eq(&request.password),
                    users_db::is_admin.eq(false),
                    users_db::duel_points.eq(0),
                ))
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
