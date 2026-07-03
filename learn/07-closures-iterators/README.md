# 07 - Closures & Iterators

函数式编程：闭包与迭代器。

## 知识点

- 闭包语法：|参数| 表达式
- 三种闭包 trait：Fn、FnMut、FnOnce
- 闭包捕获环境变量
- 迭代器 trait（Iterator）
- 适配器：map、filter、enumerate、zip、chain
- 消费者：collect、fold、sum、any、all、find
- 迭代器 vs for 循环的性能

## 与 Go/Java 的区别

| 概念 | Rust | Go | Java |
|------|------|----|------|
| 闭包 | Fn/FnMut/FnOnce | func literal | lambda |
| 迭代 | Iterator trait | for-range | Stream API |
| 惰性求值 | 默认惰性 | 无 | Stream 惰性 |
| 零成本抽象 | 编译期单态化 | 无 | 虚方法调用 |

## 代码示例

```bash
cargo run
```

输出：

```
--- Closures ---
基本闭包: 10
捕获环境: Hello, Rust!
可变闭包: counter = 5

--- Iterators ---
map: [20, 40, 60, 80, 100]
filter: [10, 20]
enumerate: [(0, 10), (1, 20), (2, 30), (3, 40), (4, 50)]
zip: [(1, 100), (2, 200), (3, 300)]

--- Consumers ---
sum: 150
any > 100: false
find first > 3: Some(10)
fold: 120
filter -> map -> sum: 12

--- Custom Iterator ---
Fibonacci: 1 1 2 3 5 8 13 21 34 55
```
