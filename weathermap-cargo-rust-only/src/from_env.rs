use serde_json::Value;
use std::collections::HashMap;
use std::env;

#[derive(Debug)]
pub struct ApiValues {
    weather_api_key: String,
    longitude: String,
    latitude: String,
    url: String,
}

impl ApiValues {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        Ok(ApiValues {
            weather_api_key: env::var("WEATHER_API_KEY")?,
            longitude: env::var("LONGITUDE")?,
            latitude: env::var("LATITUDE")?,
            url: env::var("URL_WEATHER")?,
        })
    }
}

pub async fn get_weathe_from_env() -> Result<HashMap<String, Value>, Box<dyn std::error::Error>> {
    let data = ApiValues::new()?;
    // Create a new HTTP client
    let client = reqwest::Client::new();
    // Send GET request to the weather API with the query parameters
    let resp = client
        .get(&data.url)
        .query(&[
            ("lat", &data.latitude),
            ("lon", &data.longitude),
            ("appid", &data.weather_api_key),
        ])
        .send()
        .await? // Await the HTTP response
        .json::<HashMap<String, Value>>() // Parse the JSON into a HashMap
        .await?;
    Ok(resp) // Return the parsed response
}
