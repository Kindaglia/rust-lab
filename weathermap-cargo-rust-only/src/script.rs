use crate::models::WeatherSummary;
use serde_json::Value;

pub fn make_weather_summary(data: &Value) {
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
