#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate diesel;

mod database;
mod news;
mod shop;
mod tournaments;
mod users;

use crate::database::Database;
use diesel::connection::SimpleConnection;
use rocket::routes;
use std::env;

pub struct AdminKey(String);

fn main() {
    if let Ok(admin_key) = env::var("ADMIN_KEY") {
        let rocket = rocket::ignite()
            .attach(Database::fairing())
            .manage(AdminKey(admin_key))
            .mount("/users", routes![users::list, users::profile, users::add])
            .mount(
                "/tournaments",
                routes![
                    tournaments::list,
                    tournaments::start,
                    tournaments::report_match
                ],
            );
        let db = Database::get_one(&rocket).unwrap();
        db.batch_execute(include_str!("../setup_db.sql")).unwrap();
        rocket.launch();
    } else {
        println!("Error: ADMIN_KEY environment variable not set.");
    }
}
