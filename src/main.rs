#[macro_use]
extern crate rocket;
extern crate rocket_validation;

use crate::controllers::auth_controller::login;
use crate::controllers::test_controller::test_hello;
use crate::utils::catcher::{handle_unauthorized, handle_unprocessable_entity};

pub mod auth;
mod controllers;
mod models;
pub mod service;
mod utils;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/api/auth", routes![login])
        .mount("/api/test", routes![test_hello])
        .register(
            "/",
            catchers![handle_unprocessable_entity, handle_unauthorized],
        )
}
