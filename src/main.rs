#[macro_use]
extern crate rocket;
extern crate rocket_validation;

use crate::controllers::auth_controller::login;
use crate::utils::response::handle_unprocessable_entity;

mod controllers;
mod models;
mod utils;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .mount("/api/auth", routes![login])
        .register("/", catchers![handle_unprocessable_entity,])
}
