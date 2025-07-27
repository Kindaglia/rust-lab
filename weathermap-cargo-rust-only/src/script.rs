use crate::from_env;

pub async fn test_utils_function() {
    match from_env::get_weathe_from_env().await {
        Ok(weather_data) => {
            println!("Request completed successfully!");
            println!("Weather data:\n{:#?}", weather_data); // Pretty-print the full response
        }
        Err(e) => {
            println!("Error: {}", e);
        }
    }
}
/*
*
* #[derive(Debug)]
struct WeatherSummary {
    city: String,
    temperature: f64,
    humidity: u64,
    condition: String,
    wind_speed: f64,
}

*/
