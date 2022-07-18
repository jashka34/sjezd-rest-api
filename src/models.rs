use std::time::SystemTime;
use super::schema::usrs;

#[derive(Queryable)]
pub struct Usr {
    pub id : i32,
    pub name : String,
    pub created_at : SystemTime,
    pub updated_at : SystemTime,
    pub active : bool
}

#[derive(Insertable)]
#[table_name = "usrs"]
pub struct NewUsr<'a> {
    pub name: &'a str
}

