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

#[derive(Debug, Clone, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct NewUsr {
    pub name: String 
}

impl Usr {

    pub async fn get_all(mut db: Connection<Db>) -> Result<Vec<Usr>> {
       let users = sqlx::query_as!(Usr, "SELECT * FROM usrs")
               .fetch(&mut *db)
               .try_collect::<Vec<_>>()
               .await?;
       Ok(users)
    }

    pub async fn get_user_by_id(mut db: Connection<Db>, id: i32) -> Result<Usr> {
       let user = sqlx::query_as!(Usr, "SELECT * FROM usrs WHERE id = $1", id)
               .fetch_one(&mut *db)
               .await?;
       Ok(user)
    }
    
    pub async fn add(mut db: Connection<Db>, name: String) -> Result<Usr> {
        let usr = sqlx::query_as!(
            Usr,
            "INSERT INTO usrs (name) VALUES ($1) RETURNING *",
            name
        )
        .fetch_one(&mut *db)
        .await?;

        Ok(usr)
    }

}
