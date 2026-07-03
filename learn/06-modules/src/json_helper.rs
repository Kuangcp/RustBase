pub fn format_json(name: &str, version: i32) -> String {
    format!("{{\"name\":\"{name}\",\"version\":{version}}}")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_json() {
        let result = format_json("Rust", 1);
        assert_eq!(result, r#"{"name":"Rust","version":1}"#);
    }
}
