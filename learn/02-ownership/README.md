# 02 - Ownership

Rust 核心概念：所有权、借用、生命周期。

## 知识点

- 栈 vs 堆
- 所有权规则（每个值有且只有一个所有者）
- 移动语义（Move）vs Copy
- 借用：不可变引用（&T）和可变引用（&mut T）
- 生命周期标注基础（'a）
- 悬垂引用的编译期防护

## 与 Go/Java 的区别

| 概念 | Rust | Go/Java |
|------|------|---------|
| 内存管理 | 编译期所有权 | GC |
| 字符串 | String（堆）/ &str（视图） | string（堆+GC） |
| 赋值 | 移动（默认） | 引用复制 |
| 空值 | Option<T> | null/nil |

## 代码示例

```bash
cargo run
```
