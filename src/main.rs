#[macro_use]
extern crate rocket;
extern crate rocket_validation;
extern crate serde;
extern crate sea_orm;

use rocket::fairing::AdHoc;
use sea_orm::Database;
use crate::controllers::auth_controller::login;
use crate::controllers::test_controller::test_hello;
use crate::utils::catcher::{handle_unauthorized, handle_unprocessable_entity};

pub(crate) mod auth;
pub(crate) mod controllers;
pub(crate) mod models;
pub(crate) mod service;
pub(crate) mod utils;
pub mod db;

#[launch]
fn rocket() -> _ {
    // sea_orm Entity first mode
    rocket::build()
        .attach(AdHoc::try_on_ignite("Sea-ORM Database", |rocket| async {
            match Database::connect("sqlite:todolist.db").await {
                Ok(conn) => Ok(rocket.manage(conn)),
                Err(e) => {
                    eprintln!("Failed to connect to database: {}", e);
                    Err(rocket)
                }
            }
        }))
        .mount("/api/auth", routes![login])
        .mount("/api/test", routes![test_hello])
        .register(
            "/",
            catchers![handle_unprocessable_entity, handle_unauthorized],
        )
}
