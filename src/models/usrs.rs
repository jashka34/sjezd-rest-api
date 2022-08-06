use chrono::prelude::*;
use crate::schema::usrs;
use rocket::serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Deserialize, Serialize, Queryable)]
#[serde(crate = "rocket::serde")]
pub struct Usr {
    pub id : Option<i32>,
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

