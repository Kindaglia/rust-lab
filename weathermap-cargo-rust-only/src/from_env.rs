use crate::models::ApiValues;
use serde_json::Value;
use std::env;
use std::error::Error;

impl ApiValues {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let weather_api_key = match env::var("WEATHER_API_KEY") {
            Ok(val) if !val.trim().is_empty() => val,
            _ => return Err("Missing or empty env var: WEATHER_API_KEY".into()),
        };

        let longitude = match env::var("LONGITUDE") {
            Ok(val) if !val.trim().is_empty() => val,
            _ => return Err("Missing or empty env var: LONGITUDE".into()),
        };

        let latitude = match env::var("LATITUDE") {
            Ok(val) if !val.trim().is_empty() => val,
            _ => return Err("Missing or empty env var: LATITUDE".into()),
        };

        let url = match env::var("URL_WEATHER") {
            Ok(val) if !val.trim().is_empty() => val,
            _ => return Err("Missing or empty env var: URL_WEATHER".into()),
        };

        Ok(ApiValues {
            weather_api_key,
            longitude,
            latitude,
            url,
        })
    }
}
pub async fn get_weather_from_env() -> Result<Value, Box<dyn Error>> {
    let data = ApiValues::new()?;
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
        .json::<Value>() // Instead of HashMap, parse directly as Value
        .await?;

    Ok(resp)
}
