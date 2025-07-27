use dotenv::dotenv;

mod from_env;
mod script;

#[tokio::main]
async fn main() {
    dotenv().ok(); // load value from env
    script::test_utils_function().await;
}
