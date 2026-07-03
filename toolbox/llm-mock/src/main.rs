mod config;

use config::Config;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::load("config.toml")?;
    println!("LLM Mock Server starting on {}:{}", config.server.host, config.server.port);
    Ok(())
}