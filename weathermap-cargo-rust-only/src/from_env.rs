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

pub async fn get_weathe_from_env() -> Result<(), Box<dyn std::error::Error>> {
    let data = ApiValues::new()?;
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
