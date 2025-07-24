# weathermap-cargo-rust-only

A Rust project to fetch weather data using the OpenWeatherMap API.

## Getting Started

Create a new Rust project:

```bash
cargo new weathermap-cargo-rust-only
```

## Required Environment Variables

To use this program, create a `.env` file in the project root with the following variables:

```
WEATHER_API_KEY=
LATITUDE=
LONGITUDE=
URL_WEATHER=
```

Fill in your OpenWeatherMap API key, latitude, longitude, and the weather API URL.

## OpenWeatherMap API

- [API Documentation](https://openweathermap.org/current)

### Example API Endpoints

- By coordinates:
    ```
    https://api.openweathermap.org/data/2.5/weather?lat={lat}&lon={lon}&appid={API key}
    ```
    Example:
    ```
    https://api.openweathermap.org/data/2.5/weather?lat=44.34&lon=10.99&appid={API key}
    ```

- By city name and country code:
    ```
    https://api.openweathermap.org/data/2.5/weather?q={city name},{country code}&appid={API key}
    ```


Replace `{API key}` with your OpenWeatherMap API key.

## Rust Naming Conventions

- **Variables and functions**: `snake_case`
- **Types, structs, enums, traits**: `PascalCase` (a.k.a. `UpperCamelCase`)
- **Constants and statics**: `SCREAMING_SNAKE_CASE`

### Examples

```rust
// variable
let weather_data = 42;

// function
fn fetch_weather(city_name: &str) -> WeatherData {
    // ...
}

// struct
struct WeatherData {
    temperature: f32,
    humidity: u8,
}

// enum
enum WeatherCondition {
    Clear,
    Rain,
    Snow,
}

// trait
trait Displayable {
    fn display(&self);
}

// constant
const DEFAULT_CITY: &str = "London";
```
