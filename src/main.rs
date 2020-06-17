#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate diesel;

mod database;
mod news;
mod shop;
mod tournaments;
mod users;

use crate::database::Database;
use rocket::routes;
use std::env;

pub struct AdminKey(String);

fn main() {
    if let Ok(admin_key) = env::var("ADMIN_KEY") {
        rocket::ignite()
            .attach(Database::fairing())
            .manage(AdminKey(admin_key))
            .mount("/users", routes![users::list, users::view, users::add])
            .mount(
                "/tournaments",
                routes![tournaments::list, tournaments::start],
            )
            .launch();
    } else {
        println!("Error: ADMIN_KEY environment variable not set.");
    }
}
