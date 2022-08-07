use rocket::{Rocket, Build, futures};
use chrono::prelude::*;
use crate::schema::usrs;
use rocket::serde::{Serialize, Deserialize};
use rocket_db_pools::{sqlx, Connection};

use futures::stream::TryStreamExt;

use crate::db::connection::MyDb as Db;


type Result<T, E = rocket::response::Debug<sqlx::Error>> = std::result::Result<T, E>;

#[derive(Debug, Clone, Deserialize, Serialize, Queryable)]
#[serde(crate = "rocket::serde")]
pub struct Usr {
    pub id : i32,
    pub name : String,
//    #[serde(skip_serializing)]
    pub created_at : DateTime<chrono::Utc>,
//    #[serde(skip_serializing)]
    pub updated_at : DateTime<chrono::Utc>,
    pub active : bool,
//    #[serde(skip_deserializing)]
//    #[serde(skip_serializing)]
//    pub hash_psw: String,
}

#[derive(Insertable)]
#[table_name = "usrs"]
pub struct NewUsr<'a> {
    pub name: &'a str
}

impl Usr {

    pub async fn get_all(mut db: Connection<Db>) -> Result<Vec<Usr>> {

       let users = sqlx::query_as!(Usr, "SELECT * FROM usrs")
               .fetch(&mut *db)
               .try_collect::<Vec<_>>()
               .await?;

       Ok(users)
    }
}
