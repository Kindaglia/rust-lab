use dotenv::dotenv;

mod from_env;
mod from_input;
mod models;
mod script;

#[tokio::main]
async fn main() {
    dotenv().ok(); // load value from env
    script::test_utils_function().await;
    from_input::test_input_function().await;
}
