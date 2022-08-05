use std::time::SystemTime;
use crate::schema::usrs;
use rocket::serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Deserialize, Serialize, Queryable, Insertable)]
#[serde(crate = "rocket::serde")]
pub struct Usr {
    #[serde(skip_deserializing)]
    pub id : Option<i32>,
    pub name : String,
    pub created_at : SystemTime,
    pub updated_at : SystemTime,
    pub active : bool,
    pub hash_psw: String,
}

#[derive(Insertable)]
#[table_name = "usrs"]
pub struct NewUsr<'a> {
    pub name: &'a str
}

