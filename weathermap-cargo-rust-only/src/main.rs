use dotenv::dotenv;

mod from_env; // Dichiarazione del modulo

#[tokio::main]
async fn main() {
    dotenv().ok(); // load value from env

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
