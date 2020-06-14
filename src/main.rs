#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate diesel;

mod database;
mod news;
mod shop;
mod tournaments;
mod users;

use diesel::SqliteConnection;
use rocket::routes;
use rocket_contrib::database;
use std::env;
use std::io::Write;
use std::process::{Command, Stdio};

#[database("database")]
pub struct Database(SqliteConnection);

pub struct AdminKey(String);

fn main() {
    let mut c = Command::new("sqlite3")
        .arg("ygo_server.sqlite3")
        .stdin(Stdio::piped())
        .spawn()
        .unwrap();
    c.stdin
        .as_mut()
        .unwrap()
        .write_all(include_bytes!("../setup_db.sql"))
        .unwrap();

    if let Ok(admin_key) = env::var("ADMIN_KEY") {
        rocket::ignite()
            .attach(Database::fairing())
            .manage(AdminKey(admin_key))
            .mount(
                "/users",
                routes![users::list, users::view, users::add, users::delete],
            )
            .launch();
    } else {
        println!("Error: ADMIN_KEY environment variable not set.");
    }
}
