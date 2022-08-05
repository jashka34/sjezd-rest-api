use rocket::fairing::AdHoc;
// use rocket::{Rocket, Build};

use rocket::serde::json::Json;
// use crate::schema::usrs::{self, dsl::*};
// use crate::models::usrs;
 use crate::schema::usrs;

use rocket_sync_db_pools::{database, diesel};
use diesel::prelude::*;

#[database("db")]
pub struct Db(PgConnection);

type Result<T, E = rocket::response::Debug<diesel::result::Error>> = std::result::Result<T, E>;

#[get("/users")]
pub async fn get_all_users(db: Db) -> Result<Json<Vec<Option<i32>>>> {

    let ids: Vec<Option<i32>> = 
        db.run( move |conn| {
            usrs::table
                .select(usrs::id)
                .load(conn)
        }).await?;

    Ok(Json(ids))
}

pub fn stage() -> AdHoc {
            AdHoc::on_ignite("Diesel Usrs Stage", |rocket| async {
                        rocket.attach(Db::fairing())
                              .mount("/api", routes![get_all_users]) 
            })
}
