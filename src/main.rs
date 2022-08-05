#[macro_use] extern crate rocket;
#[macro_use] extern crate diesel;
//#[macro_use] extern crate diesel_migrations;
// extern crate rocket_dyn_templates;
// #[macro_use] extern crate rocket_sync_db_pools;

pub mod models;
pub mod schema;
pub mod routes;


use rocket::fairing::AdHoc;


// use rocket::*;
use rocket_dyn_templates::{Template, context};
// use rocket_sync_db_pools::{database, diesel::PgConnection};
// use diesel::{dsl::*, prelude::*};
// use models::*;

// use dotenv::dotenv;

// #[database("db")]
// pub struct Db(PgConnection);

#[get("/")]
fn index() -> Template {
    Template::render("index", context! { name: "Alexey" })
}


#[launch]
fn rocket() -> _ {
    rocket::build()
        // .attach(Db::fairing())
        .attach(Template::fairing())
        .attach(routes::usrs::stage())
        .mount("/", routes![index])
        .mount("/config", routes![routes::config::read_config]) 
        .attach(AdHoc::config::<routes::config::AppConfig>())
}
