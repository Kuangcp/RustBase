/// 公共函数：外部可见
pub fn public_func() {
    println!("public_func: hello from utils");
    internal_func();
}

/// 内部函数：仅模块内可见（无 pub）
fn internal_func() {
    println!("internal_func: this is private");
}

/// 公共结构体，字段也需 pub 才能外部访问
pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl Point {
    // 公共方法
    pub fn new(x: f64, y: f64) -> Self {
        Point { x, y }
    }

    // 私有方法
    fn validate(&self) -> bool {
        self.x.is_finite() && self.y.is_finite()
    }

    pub fn display(&self) {
        if self.validate() {
            println!("Point({}, {})", self.x, self.y);
        }
    }
}
