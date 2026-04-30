#[macro_use]
extern crate rocket;
extern crate rocket_validation;
extern crate sea_orm;
extern crate serde;

use crate::controllers::auth_controller::login;
use crate::controllers::test_controller::test_hello;
use crate::controllers::todolist_controller::get_todolist;
use crate::utils::catcher::{handle_unauthorized, handle_unprocessable_entity};
use crate::utils::log::logging;
use rocket::fairing::AdHoc;
use sea_orm::Database;

pub(crate) mod auth;
pub(crate) mod controllers;
pub(crate) mod db;
pub(crate) mod models;
pub(crate) mod service;
pub(crate) mod utils;

#[launch]
fn rocket() -> _ {
    // 判断环境
    let is_production = std::env::var("ROCKET_ENV")
        .map(|env| env == "production")
        .unwrap_or(false);
    // 初始化日志
    logging::init_logging(is_production);
    // sea_orm Entity first mode
    rocket::build()
        .attach(AdHoc::try_on_ignite("Sea-ORM Database", |rocket| async {
            // 从 Rocket.toml 中读取 databases.db_name.url
            let db_url = match rocket
                .figment()
                .extract_inner::<String>("databases.db_name.url")
            {
                Ok(url) => url,
                Err(e) => {
                    eprintln!("Failed to read database URL from config: {}", e);
                    return Err(rocket);
                }
            };
            match Database::connect(db_url).await {
                Ok(conn) => {
                    info!("Successfully connected to database");
                    Ok(rocket.manage(conn))
                }
                Err(e) => {
                    error!("Failed to connect to database: {}", e);
                    Err(rocket)
                }
            }
        }))
        .mount("/api/auth", routes![login])
        .mount("/api/todolist", routes![get_todolist])
        .mount("/api/test", routes![test_hello])
        .register(
            "/",
            catchers![
                // rocket_validation::validation_catcher,
                handle_unprocessable_entity,
                handle_unauthorized
            ],
        )
}
