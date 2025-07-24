use dotenv::dotenv;
use std::env;
#[derive(Debug)]
// #[allow(dead_code)] // for remvoe warning 



struct ApiValues {
    weather_api_key: String,
    longitude: String,
    latitude: String,
    url: String,
}

fn init_data() -> ApiValues {
    dotenv().ok(); // load .env
    ApiValues {
        weather_api_key: env::var("WEATHER_API_KEY").expect("WEATHER_API_KEY not set"),
        longitude: env::var("LONGITUDE").expect("LONGITUDE not set"),
        latitude: env::var("LATITUDE").expect("LATITUDE not set"),
        url: env::var("URL_WEATHER").expect("URL not set"),
    }
}

fn main() {
    let data = init_data();
    println!("{data:#?}");// print  “object-like”
}
