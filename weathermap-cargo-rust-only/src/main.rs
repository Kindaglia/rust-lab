mod from_env;
mod from_input;
mod models;
mod script;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();

    loop {
        // simple menu
        const BANNER: &str = r#"
        ┌─────────────────────────────┐
        │  1  🌍  From ENV            │
        │  2  ⌨️  From INPUT          │
        │  3  👋  Exit – bye!         │
        └─────────────────────────────┘
        Select: "#;

        print!("{BANNER}");
        std::io::Write::flush(&mut std::io::stdout())?;

        let mut choice = String::new();
        std::io::stdin().read_line(&mut choice)?;
        match choice.trim() {
            "1" => match from_env::get_weather_from_env().await {
                Ok(w) => {
                    script::make_weather_summary(&w);
                }
                Err(e) => eprintln!("❌ Error: {}", e),
            },
            "2" => match from_input::get_weather_from_input().await {
                Ok(w) => {
                    script::make_weather_summary(&w);
                }
                Err(e) => eprintln!("❌ Error: {}", e),
            },
            "3" => break,
            _ => eprintln!("Invalid choice, try again."),
        }
    }
    Ok(())
}
