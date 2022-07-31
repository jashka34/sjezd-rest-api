// use rocket::serde::Deserialize;
// use rocket_sync_db_pools::diesel;
use rocket::{Rocket, Build};

use rocket::get;
use rocket::serde::{Serialize, Deserialize, json::Json};

use rocket_sync_db_pools::{database, diesel::PgConnection};
use crate::models::usrs;

#[database("db")]
pub struct Db(PgConnection);

type Result<T, E = rocket::response::Debug<diesel::result::Error>> = std::result::Result<T, E>;

#[get("/users")]
pub async fn get_all_users(db: Db) -> Result<Json<Vec<Option<i64>>>> {

    let ids: Vec<Option<i64>> = 
        // vec![ 111, 222 ];
        db.run( move |conn| {
            usrs::table
                .select(usrs.id)
                .load(conn)
        }).await?;

    Ok(Json(ids))
}
