use rocket::{Rocket, Build, futures};
use rocket::fairing::{self, AdHoc};
use rocket::response::status::Created;
use rocket::serde::{Serialize, Deserialize, json::Json};
use rocket_db_pools::{sqlx, Database, Connection};

use futures::{stream::TryStreamExt, future::TryFutureExt};

use crate::models::usrs::Usr;
// use crate::schema::usrs;

#[derive(Database)]
#[database("db")]
pub struct Db(sqlx::PgPool);

type Result<T, E = rocket::response::Debug<sqlx::Error>> = std::result::Result<T, E>;

#[get("/users")]
pub async fn get_all_users(mut db: Connection<Db>) -> Result<Json<Vec<i32>>> {
   let ids = sqlx::query!("SELECT id FROM usrs")
               .fetch(&mut *db)
               .map_ok(|record| record.id)
               .try_collect::<Vec<_>>()
               .await?;

       Ok(Json(ids))
}

#[get("/user/<id>")]
pub async fn get_user(mut db: Connection<Db>, id: i32) -> Option<Json<Usr>> {
        sqlx::query!("SELECT id, name, active FROM usrs WHERE id = $1", id)
        .fetch_one(&mut *db)
        .map_ok(|r| Json(Usr {   id:         Some(r.id)
                               , name:       r.name
                               , active:     r.active
                               // , created_at: r.created_at
                               // , updated_at: r.updated_at
                            }))
        .await
        .ok()
}

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("Diesel Usrs Stage", |rocket| async {
        rocket.attach(Db::init())
              .mount("/api", routes![get_all_users, get_user]) 
    })
}
