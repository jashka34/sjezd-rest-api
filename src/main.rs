#[macro_use] extern crate rocket;
#[macro_use] extern crate diesel;
// extern crate rocket_dyn_templates;

pub mod models;
pub mod schema;
pub mod config;

use rocket::{State, Config};
use rocket::fairing::AdHoc;
use rocket::serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(crate = "rocket::serde")]
struct AppConfig {
        key: String,
        port: u16
}

use rocket::*;
use rocket_dyn_templates::{Template, context};
use rocket_sync_db_pools::{database, diesel::PgConnection};
// use diesel::{dsl::*, prelude::*};
// use models::*;

// use dotenv::dotenv;

#[database("db")]
pub struct Db(PgConnection);

#[get("/")]
fn index() -> Template {
        Template::render("index", context! { name: "Alexey" })
}

#[get("/")]
fn read_config(rocket_config: &Config, app_config: &State<AppConfig>) -> String {
        format!("{:#?}\n{:#?}", app_config, rocket_config)
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        // .attach(Db::fairing())
        .attach(Template::fairing())
        .mount("/", routes![index])
        .mount("/config", routes![read_config]) 
        .attach(AdHoc::config::<AppConfig>())
}
