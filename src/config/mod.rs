use dotenvy::dotenv;
use std::env;

pub struct Settings {
    pub database_url: String,
}

impl Settings {
    pub fn new() -> Self {
        dotenv().ok(); // Load environment variables from .env file
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        
        Settings { database_url }
    }
}