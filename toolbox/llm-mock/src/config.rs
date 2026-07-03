use serde::Deserialize;
use std::fs;

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub server: ServerConfig,
    pub response: ResponseConfig,
    pub error_simulation: ErrorConfig,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ResponseConfig {
    pub log_file: String,
    pub stream_delay_min_ms: u64,
    pub stream_delay_max_ms: u64,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ErrorConfig {
    pub enabled: bool,
    pub rate_limit_probability: f64,
    pub timeout_probability: f64,
}

impl Config {
    pub fn load(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let content = fs::read_to_string(path)?;
        let config: Config = toml::from_str(&content)?;
        Ok(config)
    }

    pub fn load_response_content(&self) -> Result<String, Box<dyn std::error::Error>> {
        let content = fs::read_to_string(&self.response.log_file)?;
        Ok(content)
    }
}