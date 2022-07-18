#[macro_use]
extern crate diesel;
extern crate dotenv;

pub mod models;
pub mod schema;

use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;

use self::models::{Usr,NewUsr};

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connection to {}", database_url))
}

pub fn create_usr<'a>(conn: &PgConnection, name: &'a str) -> Usr {
    use schema::usrs;

    let new_usr = NewUsr { name };

    diesel::insert_into( usrs::table )
        .values(&new_usr)
        .get_result(conn)
        .expect("Error saving new usr")
}
