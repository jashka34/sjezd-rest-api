use rocket::{State, Config};
use rocket::serde::Deserialize;
use rocket::get;


#[derive(Debug, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct AppConfig {
        key: String,
        port: u16
}

#[get("/")]
pub fn read_config(rocket_config: &Config, app_config: &State<AppConfig>) -> String {
        format!("{:#?}\n{:#?}", app_config, rocket_config)
}
