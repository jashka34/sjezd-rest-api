use rocket::config::Config;
use rocket::figment::providers::{Env};
use dotenv::dotenv;

pub fn from_env() -> Config {
    
    dotenv().ok();

    let url = std::env::var("DATABASE_URL")
                .expect("database_url not found in .env");
    println!("DATABASE_URL: {}", url);
    
    let port:u16 = std::env::var("PORT")
                            .unwrap_or_else(|_| "8000".to_string())
                            .parse::<u16>()
                            .expect("PORT environment variable should parse to an integer");
    println!("PORT: {}", port);

    let address = std::env::var("ADDRESS")
                .unwrap_or_else(|_| "0.0.0.0".to_string());
    println!("ADDRESS: {}", address);

    let figment = rocket::Config::figment()
            .merge(("port", port))
            .merge(("address", address))
            .merge(("url", url));
    println!("{:?}", figment);

    Config::from(figment)
}
