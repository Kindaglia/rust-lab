use dotenv::dotenv;
use std::env;

fn get_value_from_env() {
    dotenv().ok(); // Load environment variables from a .env file, if present
    let api_key = env::var("WEATHER_API_KEY").expect("Variable not found");
    println!("API Key: {}", api_key);
}

fn main() {
    get_value_from_env();
}
