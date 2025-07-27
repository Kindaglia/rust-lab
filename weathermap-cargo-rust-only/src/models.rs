#[derive(Debug)]
pub struct ApiValues {
    pub weather_api_key: String,
    pub longitude: String,
    pub latitude: String,
    pub url: String,
}

#[derive(Debug)]
pub struct WeatherSummary {
    pub city: String,
    pub temperature: f64,
    pub humidity: u64,
    pub condition: String,
    pub wind_speed: f64,
}
