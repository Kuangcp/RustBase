fn main() {
    hello_world();
    fizz_buzz();
    temperature_conversion();
}

/// 示例1: Hello World
fn hello_world() {
    println!("--- Hello World ---");
    println!("Hello, Rust!");

    // 变量绑定（默认不可变）
    let name = "Rust";
    let version = 2024;

    // shadowing：同名变量覆盖前一个
    let version = version + 1;

    println!("Language: {name}, Year: {version}");
    println!();
}

/// 示例2: FizzBuzz
fn fizz_buzz() {
    println!("--- FizzBuzz (1-30) ---");

    for i in 1..=30 {
        let result = match (i % 3, i % 5) {
            (0, 0) => String::from("FizzBuzz"),
            (0, _) => String::from("Fizz"),
            (_, 0) => String::from("Buzz"),
            _ => i.to_string(),
        };
        print!("{result} ");
    }
    println!();
    println!();
}

/// 示例3: 温度转换（华氏 ↔ 摄氏）
fn temperature_conversion() {
    println!("--- Temperature Conversion ---");

    let temps = [32.0, 212.0, 98.6, 0.0];

    for &f in &temps {
        let c = fahrenheit_to_celsius(f);
        println!("{f}°F = {c}°C");
    }
}

fn fahrenheit_to_celsius(f: f64) -> f64 {
    (f - 32.0) * 5.0 / 9.0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fahrenheit_to_celsius() {
        assert_eq!(fahrenheit_to_celsius(32.0), 0.0);
        assert_eq!(fahrenheit_to_celsius(212.0), 100.0);
        assert!((fahrenheit_to_celsius(98.6) - 37.0).abs() < 0.01);
    }
}
