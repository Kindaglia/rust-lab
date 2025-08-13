use reqwest::Client;
use serde_json::Value;
use std::error::Error;

pub async fn get_weather_raw_data() -> Result<Value, Box<dyn Error>> {
    let city = "Rome";
    let client = Client::new(); // Create a new client instance
    let resp = client
        .get(&format!("https://wttr.in/{}", city)) // Fixed URL formatting
        .query(&[("format", "j1"), ("lang", "en")]) // Fixed query parameters
        .send()
        .await?
        .json::<Value>()
        .await?;
    // Print the weather data
    println!("Weather data for {}: {}", city, resp);
    Ok(resp)
}

pub fn test_print() {
    println!("Weather module test print function called.");
}
