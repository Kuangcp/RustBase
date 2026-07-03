mod utils;
mod network;
mod json_helper;

// 重导出：把深层路径提升到 crate 根
pub use json_helper::format_json;

fn main() {
    module_basics();
    nested_module();
    reexport_demo();
}

/// 模块基础：pub 可见性
fn module_basics() {
    println!("--- Module Basics ---");

    // 调用 utils 模块的公共函数
    utils::public_func();

    // 内部函数无法从外部访问
    // utils::internal_func(); // 编译错误

    println!();
}

/// 嵌套模块：network::http
fn nested_module() {
    println!("--- Nested Module ---");

    let req1 = network::http::Request {
        method: String::from("GET"),
        url: String::from("https://example.com"),
    };

    let req2 = network::http::Request {
        method: String::from("POST"),
        url: String::from("https://api.com/data"),
    };

    println!("{}", req1.send());
    println!("{}", req2.send());
    println!();
}

/// 重导出：通过 crate 根直接访问深层模块
fn reexport_demo() {
    println!("--- Re-export ---");

    // 使用重导出，不需要写 network::http::...
    let data = format_json("Rust", 1);
    println!("json: {data}");
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_utils_public() {
        super::utils::public_func();
    }

    #[test]
    fn test_network_request() {
        use super::network::http::Request;
        let req = Request {
            method: "GET".into(),
            url: "https://test.com".into(),
        };
        let resp = req.send();
        assert!(resp.contains("200"));
    }
}
