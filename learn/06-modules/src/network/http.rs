pub struct Request {
    pub method: String,
    pub url: String,
}

impl Request {
    pub fn send(&self) -> String {
        // 模拟 HTTP 请求
        let status = match self.method.as_str() {
            "GET" => "200 OK",
            "POST" => "201 Created",
            "PUT" => "200 OK",
            "DELETE" => "204 No Content",
            _ => "405 Method Not Allowed",
        };
        format!("HTTP {} {} -> {}", self.method, self.url, status)
    }
}
