# 09 - Concurrency

并发编程：线程、channel、Mutex、async/await 协程。

## 知识点

### 线程模型
- std::thread 创建操作系统线程
- move 闭包转移所有权到线程
- JoinHandle 等待线程完成

### 通信机制
- std::sync::mpsc 多生产者单消费者 channel
- crossbeam（可选）多生产者多消费者
- oneshot / broadcast / watch channel

### 共享状态
- Mutex\<T\> 互斥锁
- Arc\<T\> 多线程引用计数
- RwLock\<T\> 读写锁（tokio 提供）

### async/await 协程
- async fn 返回 impl Future
- .await 挂起当前任务等待结果
- tokio::spawn 分发任务到异步运行时
- tokio::select! 并发等待多个 future
- tokio::time::timeout 超时控制
- tokio::sync  异步 channel（mpsc, oneshot, broadcast）
- tokio::sync::Semaphore 并发数控制

## 与 Go/Java 的区别

| 概念 | Rust | Go | Java |
|------|------|----|------|
| 线程 | OS 线程 | goroutine（M:N） | 虚拟线程（JDK21+） |
| 异步 | async/await（编译期状态机） | 无（goroutine 足够） | CompletableFuture |
| 运行时 | 需显式选择（tokio） | 内置 | 内置 |
| 零成本 | 编译期单态化 | 运行时开销 | 虚方法调用 |
| 取消 | drop Future（协作式） | context.Done() | interrupt() |
| 挂起 | .await（状态机） | <- channel | .join() |

## 代码示例

```bash
cargo run
```

输出：

```
=== Part 1: Threading ===

--- Thread Spawn ---
线程 0 结果: 0
线程 1 结果: 1
线程 2 结果: 2

--- Channel ---
收到: hello
收到: world
收到: rust

--- Producer-Consumer ---
生产者发送 10 个任务
消费者处理: task-0
消费者处理: task-1
...
处理完成: 10 个任务

=== Part 2: Async/Await ===

--- Basic Async ---
Hello from async!
1 + 2 = 3

--- Spawn Tasks ---
task-0 完成
task-1 完成
task-2 完成
所有任务完成

--- Select ---
fast 先完成

--- Timeout ---
操作超时

--- Async Channel ---
async-recv: msg-0
async-recv: msg-1
async-recv: msg-2

--- Semaphore (并发限制) ---
task-0 完成
task-1 完成
task-2 完成
...
全部完成
```
