fn main() {
    basic_match();
    destructuring();
    if_let_demo();
    while_let_demo();
    config_parsing();
}

// ====== 基础 match ======

fn basic_match() {
    println!("--- Basic Match ---");

    let number = 2;

    match number {
        1 => println!("one"),
        2 => println!("2 is even"),
        3 => println!("three"),
        _ => println!("other"),
    }

    // match 也是表达式，可以返回值
    let description = match number {
        1 => "one",
        2 => "even",
        3 => "odd",
        _ => "other",
    };
    println!("description: {description}");
    println!();
}

// ====== 解构 ======

#[derive(Debug)]
enum Message {
    Quit,
    Write(String),
    Move { x: i32, y: i32 },
    Color(i32, i32, i32),
}

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

fn destructuring() {
    println!("--- Destructuring ---");

    // 解构结构体
    let p = Point { x: 1, y: 0 };
    match p {
        Point { x: 0, y } => println!("on y-axis at {y}"),
        Point { x, y: 0 } => println!("{p:?} is on the x-axis"),
        Point { x, y } => println!("at ({x}, {y})"),
    }

    // 解构枚举
    let msgs = vec![
        Message::Quit,
        Message::Write(String::from("hello")),
        Message::Move { x: 10, y: 20 },
        Message::Color(0, 128, 255),
    ];

    for msg in &msgs {
        match msg {
            Message::Quit => println!("Message::Quit"),
            Message::Write(text) => println!("Message::Write(\"{text}\")"),
            Message::Move { x, y } => println!("Message::Move {{ x: {x}, y: {y} }}"),
            Message::Color(r, g, b) => println!("Message::Color({r}, {g}, {b})"),
        }
    }

    // 解构元组
    let pair = (0, -2);
    match pair {
        (0, 0) => println!("origin"),
        (x, 0) => println!("on x-axis at {x}"),
        (0, y) => println!("on y-axis at {y}"),
        (x, y) => println!("at ({x}, {y})"),
    }
    println!();
}

// ====== if let ======

fn if_let_demo() {
    println!("--- If Let ---");

    let config_value: Option<i32> = Some(42);

    // 只关心一种情况时，if let 比 match 简洁
    if let Some(value) = config_value {
        println!("Got value: {value}");
    }

    // 等价的 match 写法
    match config_value {
        Some(value) => println!("Got value: {value}"),
        None => {}
    }
    println!();
}

// ====== while let ======

fn while_let_demo() {
    println!("--- While Let ---");

    let mut stack = vec![1, 2, 3, 4, 5];

    // 只要 pop() 返回 Some 就继续循环
    while let Some(top) = stack.pop() {
        print!("{top} ");
    }
    println!();
    println!();
}

// ====== 实战：配置解析 ======

#[derive(Debug)]
struct Config {
    host: String,
    port: u16,
    verbose: bool,
}

fn config_parsing() {
    println!("--- Config Parsing ---");

    let lines = vec![
        "host=localhost",
        "port=8080",
        "verbose=true",
        "debug=off",
    ];

    let mut host = String::new();
    let mut port = 0u16;
    let mut verbose = false;

    for line in &lines {
        // 解构 "key=value" 格式
        let parts: Vec<&str> = line.splitn(2, '=').collect();
        if let [key, value] = parts.as_slice() {
            match *key {
                "host" => host = value.to_string(),
                "port" => port = value.parse().unwrap_or(0),
                "verbose" => verbose = *value == "true",
                _ => println!("Unknown option: {key}"),
            }
        }
    }

    println!("Parsed: host={host}, port={port}, verbose={verbose}");

    // match guard：带条件的匹配
    let number = 42;
    match number {
        n if n < 0 => println!("{n} is negative"),
        n if n % 2 == 0 => println!("{n} is even"),
        n => println!("{n} is odd"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_match_even_odd() {
        for (input, expected) in [(1, "odd"), (2, "even"), (3, "odd"), (4, "even")] {
            let result = match input {
                n if n % 2 == 0 => "even",
                _ => "odd",
            };
            assert_eq!(result, expected);
        }
    }

    #[test]
    fn test_destructure_point() {
        let p = Point { x: 0, y: 5 };
        let desc = match p {
            Point { x: 0, y } => format!("y={y}"),
            Point { x, y: 0 } => format!("x={x}"),
            _ => format!("both"),
        };
        assert_eq!(desc, "y=5");
    }

    #[test]
    fn test_message_variant() {
        let msg = Message::Write(String::from("hi"));
        match msg {
            Message::Write(t) => assert_eq!(t, "hi"),
            _ => panic!("wrong variant"),
        }
    }
}
