use rocket::{/*Rocket,Build, */ futures};
use rocket::fairing::{self, AdHoc};
// use rocket::response::status::Created;
use rocket::serde::{Serialize, Deserialize, json::Json};
use rocket_db_pools::{sqlx, Connection, Database};
use rocket::http::Status;

use futures::{stream::TryStreamExt, future::TryFutureExt};

use crate::models::usrs::Usr;
use crate::db::connection::MyDb as Db;
// use crate::schema::usrs;


type Result<T, E = rocket::response::Debug<sqlx::Error>> = std::result::Result<T, E>;

#[get("/user_ids")]
pub async fn get_all_users_ids(mut db: Connection<Db>) -> Result<Json<Vec<i32>>> {
   let ids = sqlx::query!("SELECT id FROM usrs")
               .fetch(&mut *db)
               .map_ok(|record| record.id)
               .try_collect::<Vec<_>>()
               .await?;

       Ok(Json(ids))
}

#[get("/users")]
pub async fn get_all_users(db: Connection<Db>) -> Result<Json<Vec<Usr>>, Status> {
    let users = Usr::get_all(db).await;

    match users {
        Ok(users) => Ok(Json(users)),
        _ => Err(Status::NotFound),
    }

}

#[get("/user/<id>")]
pub async fn get_user(mut db: Connection<Db>, id: i32) -> Option<Json<Usr>> {
    // dbg!("asdfasdf");
    sqlx::query!("SELECT id, name, active, created_at, updated_at FROM usrs WHERE id = $1", id)
        .fetch_one(&mut *db)
        .map_ok(|r| Json(Usr {   id:         r.id
                               , name:       r.name
                               , active:     r.active
                               , created_at: r.created_at
                               , updated_at: r.updated_at
                            }))
        .await
        .ok()
}

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("Diesel Usrs Stage", |rocket| async {
        rocket.attach(Db::init())
              .mount("/api", routes![get_all_users_ids, get_all_users, get_user]) 
    })
}
