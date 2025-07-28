use crate::models::ApiValuesForInput;
use serde_json::Value;
use std::env;
use std::error::Error;
use std::io::{self, Write};

impl ApiValuesForInput {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let weather_api_key = match env::var("WEATHER_API_KEY") {
            Ok(val) if !val.trim().is_empty() => val,
            _ => return Err("Missing or empty env var: WEATHER_API_KEY".into()),
        };

        let url = match env::var("URL_WEATHER") {
            Ok(val) if !val.trim().is_empty() => val,
            _ => return Err("Missing or empty env var: URL_WEATHER".into()),
        };

        Ok(ApiValuesForInput {
            weather_api_key,
            url,
        })
    }
}

pub async fn get_weather_from_input() -> Result<Value, Box<dyn Error>> {
    println!("Input function executed successfully.");

    print!("Enter the city name: ");
    io::stdout().flush()?; // force the text to appear
    let mut city = String::new();
    io::stdin().read_line(&mut city)?;
    let city = city.trim();
    // Load configuration from environment.
    let data = ApiValuesForInput::new()?; // propagates env-var errors
    // Build and send the HTTP request.
    let client = reqwest::Client::new();
    let resp = client
        .get(&data.url)
        .query(&[("q", city), ("appid", &data.weather_api_key)])
        .send()
        .await?
        .json::<Value>()
        .await?;
    Ok(resp)
}
