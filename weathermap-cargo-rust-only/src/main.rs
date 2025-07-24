use dotenv::dotenv;
use std::env;
use std::collections::HashMap;

#[derive(Debug)]
// #[allow(dead_code)] // for remove warning
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


async fn get_weathe_from_env() -> Result<(), Box<dyn std::error::Error>> {
    let data = init_data();
    println!("{data:#?}"); // print "object-like"

    let client = reqwest::Client::new();
    let resp = client
        .get(&data.url)
        .query(&[
            ("lat", &data.latitude),
            ("lon", &data.longitude),
            ("appid", &data.weather_api_key),
        ])
        .send()
        .await?
        .json::<HashMap<String, serde_json::Value>>()
        .await?;

    println!("{resp:#?}");
    Ok(())
}

#[tokio::main]
async fn main() {
    match get_weathe_from_env().await {
        Ok(_) => println!("Richiesta completata con successo!"),
        Err(e) => println!("Errore: {}", e),
    }
}