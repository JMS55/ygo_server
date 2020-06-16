#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate diesel;

mod database;
mod news;
mod shop;
mod tournaments;
mod users;

use diesel::{sql_query, RunQueryDsl, SqliteConnection};
use rocket::routes;
use rocket_contrib::database;
use std::env;

#[database("database")]
pub struct Database(SqliteConnection);

pub struct AdminKey(String);

fn main() {
    if let Ok(admin_key) = env::var("ADMIN_KEY") {
        let rocket = rocket::ignite()
            .attach(Database::fairing())
            .manage(AdminKey(admin_key))
            .mount(
                "/users",
                routes![users::list, users::view, users::add, users::delete],
            );
        let db = Database::get_one(&rocket).unwrap();
        sql_query(include_str!("../setup_db.sql"))
            .execute(&*db)
            .unwrap();
        rocket.launch();
    } else {
        println!("Error: ADMIN_KEY environment variable not set.");
    }
}
