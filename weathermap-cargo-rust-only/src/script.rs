use crate::from_env;
use serde_json::Value;

#[derive(Debug)]
struct WeatherSummary {
    city: String,
    temperature: f64,
    humidity: u64,
    condition: String,
    wind_speed: f64,
}

pub async fn test_utils_function() {
    match from_env::get_weathe_from_env().await {
        Ok(weather_data) => {
            // println!("Request completed successfully!");
            // println!("Weather data:\n{:#?}", weather_data); // Pretty-print the full response

            // convert to HashMap in Value
            let json_value = serde_json::to_value(&weather_data).unwrap();
            make_weather_summary(&json_value);
        }
        Err(e) => {
            println!("Error: {}", e);
        }
    }
}

fn make_weather_summary(data: &Value) {
    // Create  WeatherSummary from JSON
    let weather_summary = WeatherSummary {
        city: data["name"].as_str().unwrap_or("Unknown").to_string(),
        temperature: data["main"]["temp"].as_f64().unwrap_or(0.0) - 273.15, // convert from Kelvin to Celsius
        humidity: data["main"]["humidity"].as_u64().unwrap_or(0),
        condition: data["weather"][0]["description"]
            .as_str()
            .unwrap_or("Unknown")
            .to_string(),
        wind_speed: data["wind"]["speed"].as_f64().unwrap_or(0.0),
    };

    println!("{:?}", weather_summary);
}
