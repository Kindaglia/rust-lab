use dotenv::dotenv;

mod from_env;
mod from_input;
mod models;
mod script;

#[tokio::main]
async fn main() {
    dotenv().ok(); // load value from env

    match from_env::get_weather_from_env().await {
        Ok(weather_data) => script::make_weather_summary(&weather_data),
        Err(e) => println!("❌ Error: {}", e),
    }
    match from_input::get_weather_from_input().await {
        Ok(weather_data) => script::make_weather_summary(&weather_data),
        Err(e) => println!("❌ Error: {}", e),
    }
}
