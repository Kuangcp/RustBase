# 06 - Modules

模块系统：mod、use、pub、crate、文件组织。

## 知识点

- mod 定义模块
- pub 可见性控制
- use 引入路径
- crate / self / super 路径前缀
- 文件组织：一个文件 = 一个模块（或 mod.rs）
- Cargo.toml 依赖管理
- 重导出（pub use）

## 文件结构

```
06-modules/
├── Cargo.toml
├── src/
│   ├── main.rs        # crate 根，声明子模块
│   ├── utils.rs       # utils 模块
│   └── network/       # network 模块（目录形式）
│       ├── mod.rs
│       └── http.rs
└── README.md
```

## 代码示例

```bash
cargo run
```

输出：

```
--- Module Basics ---
public_func: hello from utils
internal_func: this is private

--- Nested Module ---
HTTP GET https://example.com -> 200 OK
HTTP POST https://api.com/data -> 201 Created

--- Re-export ---
json: {"name":"Rust","version":1}
```
