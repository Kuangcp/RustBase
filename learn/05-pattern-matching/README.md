# 05 - Pattern Matching

模式匹配：match、if let、解构、match guard。

## 知识点

- match 表达式（穷尽性检查）
- if let / while let 简洁匹配
- 解构结构体、枚举、元组
- match guard（条件守卫）
- _ 通配符与绑定
- matches! 宏

## 与 Go/Java 的区别

| 概念 | Rust | Go | Java |
|------|------|----|------|
| 模式匹配 | match（穷尽） | switch（fallthrough） | switch（不穷尽） |
| 解构 | 原生支持 | 不支持 | switch 有限支持 |
| 守卫 | match guard | 无 | case 支持部分 |
| 空值匹配 | Option/Result | nil 判空 | null 判空 |

## 代码示例

```bash
cargo run
```

输出：

```
--- Basic Match ---
2 is even

--- Destructuring ---
Point { x: 1, y: 0 } is on the x-axis
Message::Quit
Message::Write("hello")
Message::Move { x: 10, y: 20 }
Message::Color(0, 128, 255)

--- If Let ---
Got value: 42

--- While Let ---
1 2 3 4 5

--- Config Parsing ---
Parsed: host=localhost, port=8080, verbose=true
Unknown option: debug
```
