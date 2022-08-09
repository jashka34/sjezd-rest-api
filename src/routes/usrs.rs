use rocket::{/*Rocket,Build, */ futures};
use rocket::fairing::{self, AdHoc};
use rocket::response::status::Created;
use rocket::serde::{Serialize, Deserialize, json::Json};
use rocket_db_pools::{sqlx, Connection, Database};
use rocket::http::Status;

use futures::{stream::TryStreamExt, future::TryFutureExt};

use crate::models::usrs::{Usr, NewUsr};
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
pub async fn get_user(db: Connection<Db>, id: i32) -> Result<Json<Usr>, Status> {
    // dbg!("asdfasdf");
    let user = Usr::get_user_by_id(db, id).await;
    match user {
        Ok(user) => Ok(Json(user)),
        _ => Err(Status::NotFound),
    }
}

#[post("/user/add", format = "application/json", data = "<request>")]
pub async fn add_user(db: Connection<Db>, request: Json<NewUsr>) -> Result<Created<Json<Usr>>, Status> {
    let usr = Usr::add(db, request.name.to_owned()).await;
    match usr {
        Ok(usr) => Ok(Created::new("/").body(Json(usr))),
        _ => Err(Status::BadRequest),
    }
}

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("Diesel Usrs Stage", |rocket| async {
        rocket.attach(Db::init())
              .mount("/api", routes![get_all_users_ids, 
                                     get_all_users, 
                                     get_user,
                                     add_user,
                                    ]
              ) 
    })
}
