use dotenv::dotenv;
use std::env;

pub fn config() -> (String, String) {
    dotenv().ok();
    let apikey = env::var("APIKEY").expect("No APIKEY found in the .ENV file!");
    let db_url = env::var("DATABASE_URL").expect("No DATABASE_URL found in the .ENV file!");
    (apikey, db_url)
    
}