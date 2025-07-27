use dotenv::dotenv;

mod from_env; // Dichiarazione del modulo

#[tokio::main]
async fn main() {
    dotenv().ok(); // load value from env

    match from_env::get_weathe_from_env().await {
        Ok(_) => println!("Richiesta completata con successo!"),
        Err(e) => println!("Errore: {}", e),
    }
}
