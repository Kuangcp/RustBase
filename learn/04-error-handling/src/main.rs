use std::fmt;
use std::fs;
use std::num::ParseIntError;

fn main() {
    option_demo();
    result_demo();
    error_propagation_demo();
    custom_error_demo();
}

// ====== Option<T> 示例 ======

fn option_demo() {
    println!("--- Option Demo ---");

    let some_number: Option<i32> = Some(42);
    let no_number: Option<i32> = None;

    println!("{some_number:?}");
    println!("{no_number:?}");

    // match 处理 Option
    match some_number {
        Some(n) => println!("找到了: {n}"),
        None => println!("没有值"),
    }

    // unwrap_or 提供默认值
    let value = no_number.unwrap_or(0);
    println!("默认值: {value}");
    println!();
}

// ====== Result<T, E> 示例 ======

fn result_demo() {
    println!("--- Result Demo ---");

    let good: Result<i32, String> = Ok(42);
    let bad: Result<i32, String> = Err(String::from("division by zero"));

    println!("{good:?}");
    println!("{bad:?}");
    println!();
}

/// 除法：返回 Result
fn divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err(String::from("division by zero"))
    } else {
        Ok(a / b)
    }
}

// ====== ? 操作符错误传播 ======

/// 读取文件并返回前 n 个字符
fn read_file_preview(path: &str, n: usize) -> Result<String, std::io::Error> {
    let content = fs::read_to_string(path)?; // ? 自动传播 io::Error
    Ok(content.chars().take(n).collect())
}

fn error_propagation_demo() {
    println!("--- Error Propagation ---");

    // 创建一个临时文件用于演示
    let temp_path = "/tmp/rust_demo.txt";
    fs::write(temp_path, "# This is a demo file for error handling.\nMore content here.").unwrap();

    match read_file_preview(temp_path, 50) {
        Ok(content) => println!("File content (first 50 chars): {content}"),
        Err(e) => println!("读取失败: {e}"),
    }

    // 不存在的文件
    match read_file_preview("/tmp/nonexistent.txt", 50) {
        Ok(content) => println!("File content: {content}"),
        Err(e) => println!("读取失败: {e}"),
    }
    println!();
}

// ====== 自定义错误类型 ======

#[derive(Debug)]
enum ConfigError {
    MissingField(String),
    InvalidPort(ParseIntError),
    InvalidHost(String),
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConfigError::MissingField(field) => write!(f, "missing field: {field}"),
            ConfigError::InvalidPort(e) => write!(f, "invalid port number: {e}"),
            ConfigError::InvalidHost(host) => write!(f, "invalid host: {host}"),
        }
    }
}

impl std::error::Error for ConfigError {}

impl From<ParseIntError> for ConfigError {
    fn from(e: ParseIntError) -> Self {
        ConfigError::InvalidPort(e)
    }
}

#[derive(Debug)]
struct Config {
    host: String,
    port: u16,
}

fn parse_config(host: &str, port_str: &str) -> Result<Config, ConfigError> {
    if host.is_empty() {
        return Err(ConfigError::MissingField("host".into()));
    }

    let port: u16 = port_str.parse()?; // ParseIntError 通过 From 转为 ConfigError

    Ok(Config {
        host: host.to_string(),
        port,
    })
}

fn custom_error_demo() {
    println!("--- Custom Error ---");

    match parse_config("localhost", "8080") {
        Ok(config) => println!("Parsed: {config:?}"),
        Err(e) => println!("Failed: {e}"),
    }

    match parse_config("", "8080") {
        Ok(config) => println!("Parsed: {config:?}"),
        Err(e) => println!("Failed to parse: {e}"),
    }

    match parse_config("localhost", "not_a_number") {
        Ok(config) => println!("Parsed: {config:?}"),
        Err(e) => println!("Failed to parse: {e}"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_divide() {
        assert_eq!(divide(10.0, 2.0), Ok(5.0));
        assert!(divide(10.0, 0.0).is_err());
    }

    #[test]
    fn test_option() {
        let x: Option<i32> = Some(5);
        assert_eq!(x.unwrap_or(0), 5);

        let y: Option<i32> = None;
        assert_eq!(y.unwrap_or(0), 0);
    }

    #[test]
    fn test_parse_config_valid() {
        let config = parse_config("localhost", "8080").unwrap();
        assert_eq!(config.host, "localhost");
        assert_eq!(config.port, 8080);
    }

    #[test]
    fn test_parse_config_missing_host() {
        assert!(parse_config("", "8080").is_err());
    }

    #[test]
    fn test_parse_config_invalid_port() {
        assert!(parse_config("localhost", "abc").is_err());
    }
}
