# 04 - Error Handling

Rust 错误处理：Result、Option、? 操作符、自定义错误。

## 知识点

- panic! 不可恢复错误
- Result<T, E> 可恢复错误
- Option<T> 空值处理
- ? 操作符自动传播错误
- unwrap / expect（仅用于原型）
- 自定义错误类型（实现 Display + Error）

## 与 Go/Java 的区别

| 概念 | Rust | Go | Java |
|------|------|----|----|
| 错误返回 | Result<T, E> | (T, error) | try/catch |
| 空值 | Option<T> | nil | null |
| 传播 | ? 操作符 | if err != nil | throws |
| panic | 类似 panic | log.Fatal | Error/Error |

## 代码示例

```bash
cargo run
```

输出：

```
--- Option Demo ---
Some(42)
None

--- Result Demo ---
Ok(42)
Err(division by zero)

--- Error Propagation ---
File content (first 50 chars): # This is a demo file...

--- Custom Error ---
Parsed: Config { host: "localhost", port: 8080 }
Failed to parse: invalid port number
```
