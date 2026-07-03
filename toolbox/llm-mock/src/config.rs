use serde::Deserialize;
use std::fs;

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    #[serde(default)]
    pub server: ServerConfig,
    #[serde(default)]
    pub response: ResponseConfig,
    #[serde(default)]
    pub error_simulation: ErrorConfig,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ServerConfig {
    #[serde(default = "default_host")]
    pub host: String,
    #[serde(default = "default_port")]
    pub port: u16,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ResponseConfig {
    #[serde(default = "default_log_file")]
    pub log_file: String,
    #[serde(default = "default_stream_delay_min")]
    pub stream_delay_min_ms: u64,
    #[serde(default = "default_stream_delay_max")]
    pub stream_delay_max_ms: u64,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ErrorConfig {
    #[serde(default)]
    pub enabled: bool,
    #[serde(default)]
    pub rate_limit_probability: f64,
    #[serde(default)]
    pub timeout_probability: f64,
}

fn default_host() -> String {
    "0.0.0.0".to_string()
}

fn default_port() -> u16 {
    8080
}

fn default_log_file() -> String {
    "mock-llm-resp.log".to_string()
}

fn default_stream_delay_min() -> u64 {
    10
}

fn default_stream_delay_max() -> u64 {
    50
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            host: default_host(),
            port: default_port(),
        }
    }
}

impl Default for ResponseConfig {
    fn default() -> Self {
        Self {
            log_file: default_log_file(),
            stream_delay_min_ms: default_stream_delay_min(),
            stream_delay_max_ms: default_stream_delay_max(),
        }
    }
}

impl Default for ErrorConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            rate_limit_probability: 0.0,
            timeout_probability: 0.0,
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            server: ServerConfig::default(),
            response: ResponseConfig::default(),
            error_simulation: ErrorConfig::default(),
        }
    }
}

impl Config {
    pub fn load(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let content = match fs::read_to_string(path) {
            Ok(content) => content,
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
                println!("Config file not found, using defaults");
                return Ok(Config::default());
            }
            Err(e) => return Err(e.into()),
        };
        let config: Config = toml::from_str(&content)?;
        Ok(config)
    }

    pub fn load_response_content(&self) -> Result<String, Box<dyn std::error::Error>> {
        match fs::read_to_string(&self.response.log_file) {
            Ok(content) => Ok(content),
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
                println!(
                    "Response file '{}' not found, using default response",
                    self.response.log_file
                );
                Ok(default_response_content())
            }
            Err(e) => Err(e.into()),
        }
    }
}

fn default_response_content() -> String {
    "Hello! This is a mock response from llm-mock.".to_string()
}
