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
    match from_env::get_weather_from_env().await {
        Ok(weather_data) => make_weather_summary(&weather_data),
        Err(e) => println!("❌ Error: {}", e),
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

    println!(
        "📍 City: {}\n🌡️ Temperature: {:.1}°C\n💧 Humidity: {}%\n🌤️ Condition: {}\n💨 Wind Speed: {:.1} m/s",
        weather_summary.city,
        weather_summary.temperature,
        weather_summary.humidity,
        weather_summary.condition,
        weather_summary.wind_speed,
    );
}

/*
* pub async fn test_utils_function() -> Result<(), Box<dyn std::error::Error>> {
    let weather_data = from_env::get_weather_from_env().await?;
    make_weather_summary(&weather_data);
    Ok(())
}

*/
