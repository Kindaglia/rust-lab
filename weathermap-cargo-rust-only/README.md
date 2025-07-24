# weathermap-cargo-rust-only

A Rust project to fetch weather data using the OpenWeatherMap API.

## Getting Started

Create a new Rust project:

```bash
cargo new weathermap-cargo-rust-only
```

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
