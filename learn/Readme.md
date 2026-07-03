# Rust Learn

Rust 系统学习路径，每个目录是一个知识模块，包含可运行的代码示例。

## 学习路径

```
基础语法 (01-06) → 函数式特性 (07) → 内存模型 (08) → 并发 (09) → 实战应用 (10-15)
```

## 模块总览

| # | 模块 | 主题 | 前置 |
|---|------|------|------|
| 01 | [hello-rust](learn/01-hello-rust/) | 环境搭建、基本语法、Cargo | — |
| 02 | [ownership](learn/02-ownership/) | 所有权、借用、生命周期 | 01 |
| 03 | [types](learn/03-types/) | 结构体、枚举、泛型、trait | 02 |
| 04 | [error-handling](learn/04-error-handling/) | Result、Option、错误传播 | 03 |
| 05 | [pattern-matching](learn/05-pattern-matching/) | match、解构、if let | 03 |
| 06 | [modules](learn/06-modules/) | 模块系统、可见性、crate | 03 |
| 07 | [closures-iterators](learn/07-closures-iterators/) | 闭包 + 迭代器 | 03 |
| 08 | [smart-pointers](learn/08-smart-pointers/) | Box、Rc、RefCell、Arc | 02 |
| 09 | [concurrency](learn/09-concurrency/) | 线程、channel、Mutex、async 基础 | 08 |
| 10 | [io](learn/10-io/) | 文件、网络、缓冲区 | 04 |
| 11 | [macros](learn/11-macros/) | 声明宏、过程宏 | 06 |
| 12 | [testing](learn/12-testing/) | 单元测试、集成测试、bench | 06 |
| 13 | [serde](learn/13-serde/) | 序列化、JSON、配置 | 04 |
| 14 | [web](learn/14-web/) | axum/actix-web 实战 | 09, 13 |
| 15 | [practice](learn/15-practice/) | 综合项目 | 01-14 |

## 模块详情

### 01-hello-rust — 入门
- 安装 rustup、cargo 基础命令（build, run, test, doc）
- `fn main()`、变量绑定、mut、常量
- 基本类型：i32, f64, bool, char, &str, String
- 格式化输出 `println!`
- **代码示例：** hello world、FizzBuzz、温度转换

### 02-ownership — 所有权（Rust 核心）
- 栈 vs 堆、所有权规则
- 移动语义 vs Copy
- 借用（&、&mut）规则
- 生命周期标注基础
- **代码示例：** 字符串所有权演示、借用 vs 移动

### 03-types — 类型系统
- 结构体（含方法 impl）
- 枚举（含枚举变体数据）
- 泛型基础
- trait 基础（derive、常用 trait）
- **代码示例：** Card 游戏、Shape 计算面积

### 04-error-handling — 错误处理
- panic! vs Result
- Option 和 unwrap
- ? 操作符
- 自定义错误类型
- **代码示例：** 文件读取错误处理、除零处理

### 05-pattern-matching — 模式匹配
- match 表达式
- if let / while let
- 解构结构体、枚举、元组
- match guard、绑定
- **代码示例：** 配置解析、命令行参数处理

### 06-modules — 模块系统
- mod、use、pub
- crate vs mod
- 文件组织与路径
- Cargo workspace 基础
- **代码示例：** 多文件项目结构

### 07-closures-iterators — 函数式编程
- 闭包（Fn、FnMut、FnOnce）
- 迭代器 trait
- 适配器（map、filter、zip）
- 消费者（collect、fold、sum）
- **代码示例：** 数据转换管道、自定义迭代器

### 08-smart-pointers — 智能指针
- Box（递归类型）
- Rc / Arc（引用计数）
- RefCell / Mutex（内部可变性）
- Deref、Drop trait
- **代码示例：** 链表实现、共享状态模式

### 09-concurrency — 并发
- std::thread 创建线程
- channel（mpsc）
- Mutex、Arc
- Send / Sync trait
- async/await 基础
- **代码示例：** 多线程下载、生产者-消费者

### 10-io — I/O 操作
- std::io trait
- 文件读写（BufReader/BufWriter）
- TCP/UDP 网络基础
- 标准输入输出
- **代码示例：** 日志解析器、简易 echo server

### 11-macros — 宏
- 声明宏 macro_rules!
- 过程宏基础（derive、属性宏）
- 常用标准库宏原理
- **代码示例：** 自定义 vec!、Builder 模式宏

### 12-testing — 测试
- #[test]、assert 宏
- 集成测试（tests/ 目录）
- 单测组织（#[cfg(test)]）
- cargo test 运行与过滤
- **代码示例：** 为前序模块代码补充测试

### 13-serde — 序列化
- serde 框架
- JSON（serde_json）
- TOML/YAML 配置
- 自定义序列化
- **代码示例：** JSON API 解析、配置文件读写

### 14-web — Web 开发
- axum 框架入门
- 路由、中间件、状态共享
- JSON 请求/响应
- 数据库集成（sqlx）
- **代码示例：** RESTful TODO API

### 15-practice — 综合项目
- CLI 工具（clap）
- 结合前序模块知识
- **代码示例：** 完整的文件管理器或聊天室
