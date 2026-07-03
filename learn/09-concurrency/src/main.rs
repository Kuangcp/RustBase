use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::Duration;

#[tokio::main]
async fn main() {
    println!("=== Part 1: Threading ===\n");
    thread_spawn();
    channel_demo();
    producer_consumer().await;

    println!("\n=== Part 2: Async/Await ===\n");
    basic_async().await;
    spawn_tasks().await;
    select_demo().await;
    timeout_demo().await;
    async_channel_demo().await;
    semaphore_demo().await;
}

// ==================== Part 1: Threading ====================

/// 线程创建与 join
fn thread_spawn() {
    println!("--- Thread Spawn ---");

    let mut handles = vec![];

    for i in 0..3 {
        let handle = thread::spawn(move || {
            // move 闭包捕获 i 的所有权
            i * i
        });
        handles.push((i, handle));
    }

    for (i, handle) in handles {
        let result = handle.join().unwrap();
        println!("线程 {i} 结果: {result}");
    }
    println!();
}

/// mpsc channel：多生产者单消费者
fn channel_demo() {
    println!("--- Channel ---");

    let (tx, rx) = mpsc::channel();

    // 多个生产者
    for msg in ["hello", "world", "rust"] {
        let tx = tx.clone();
        thread::spawn(move || {
            tx.send(msg).unwrap();
        });
    }
    drop(tx); // 关闭最后一个 sender

    // 消费者接收（迭代器自动等待所有 sender 关闭）
    for msg in rx {
        println!("收到: {msg}");
    }
    println!();
}

/// 生产者-消费者模式（用 Arc<Mutex> 共享计数器）
async fn producer_consumer() {
    println!("--- Producer-Consumer ---");

    let (tx, rx) = mpsc::channel::<String>();
    let rx = Arc::new(Mutex::new(rx));

    // 启动消费者线程
    let mut handles = vec![];
    for id in 0..2 {
        let rx = Arc::clone(&rx);
        handles.push(thread::spawn(move || loop {
            let msg = rx.lock().unwrap().recv();
            match msg {
                Ok(task) => println!("消费者处理: {task}"),
                Err(_) => break, // channel 关闭
            }
        }));
    }

    // 生产者发送任务
    println!("生产者发送 10 个任务");
    for i in 0..10 {
        tx.send(format!("task-{i}")).unwrap();
        tokio::time::sleep(Duration::from_millis(50)).await;
    }
    drop(tx);

    for h in handles {
        h.join().unwrap();
    }
    println!("处理完成: 10 个任务");
}

// ==================== Part 2: Async/Await ====================

/// 基础 async 函数
async fn fetch_data(id: u32) -> String {
    // 模拟异步 IO 操作
    tokio::time::sleep(Duration::from_millis(10)).await;
    format!("data-{id}")
}

async fn basic_async() {
    println!("--- Basic Async ---");

    // .await 等待 future 完成
    let result = add_async(1, 2).await;
    println!("1 + 2 = {result}");

    // 多个 future 顺序执行
    let d1 = fetch_data(1).await;
    let d2 = fetch_data(2).await;
    println!("顺序: {d1}, {d2}");
    println!();
}

async fn add_async(a: i32, b: i32) -> i32 {
    tokio::time::sleep(Duration::from_millis(1)).await;
    a + b
}

/// tokio::spawn：并发执行多个任务
async fn spawn_tasks() {
    println!("--- Spawn Tasks ---");

    let mut handles = vec![];

    for i in 0..3 {
        handles.push(tokio::spawn(async move {
            tokio::time::sleep(Duration::from_millis(10)).await;
            println!("task-{i} 完成");
            i
        }));
    }

    // 等待所有任务完成
    for handle in handles {
        handle.await.unwrap();
    }
    println!("所有任务完成");
    println!();
}

/// tokio::select!：并发等待多个 future，取先完成的
async fn select_demo() {
    println!("--- Select ---");

    let fast = async {
        tokio::time::sleep(Duration::from_millis(10)).await;
        "fast"
    };

    let slow = async {
        tokio::time::sleep(Duration::from_millis(100)).await;
        "slow"
    };

    tokio::select! {
        result = fast => println!("{result} 先完成"),
        result = slow => println!("{result} 先完成"),
    }
    println!();
}

/// 超时控制
async fn timeout_demo() {
    println!("--- Timeout ---");

    let slow_op = async {
        tokio::time::sleep(Duration::from_secs(5)).await;
        "done"
    };

    match tokio::time::timeout(Duration::from_millis(50), slow_op).await {
        Ok(result) => println!("结果: {result}"),
        Err(_) => println!("操作超时"),
    }
    println!();
}

/// 异步 channel（tokio::sync::mpsc）
async fn async_channel_demo() {
    println!("--- Async Channel ---");

    let (tx, mut rx) = tokio::sync::mpsc::channel(32);

    tokio::spawn(async move {
        for i in 0..3 {
            tx.send(format!("msg-{i}")).await.unwrap();
            tokio::time::sleep(Duration::from_millis(10)).await;
        }
    });

    while let Some(msg) = rx.recv().await {
        println!("async-recv: {msg}");
    }
    println!();
}

/// Semaphore：限制并发数
async fn semaphore_demo() {
    println!("--- Semaphore (并发限制) ---");

    let semaphore = Arc::new(tokio::sync::Semaphore::new(3));
    let mut handles = vec![];

    for i in 0..6 {
        let sem = Arc::clone(&semaphore);
        handles.push(tokio::spawn(async move {
            let _permit = sem.acquire().await.unwrap();
            println!("task-{i} 开始 (并发数限制为 3)");
            tokio::time::sleep(Duration::from_millis(20)).await;
            println!("task-{i} 完成");
        }));
    }

    for h in handles {
        h.await.unwrap();
    }
    println!("全部完成");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_add_async() {
        assert_eq!(add_async(2, 3).await, 5);
    }

    #[tokio::test]
    async fn test_fetch_data() {
        let data = fetch_data(1).await;
        assert_eq!(data, "data-1");
    }

    #[tokio::test]
    async fn test_spawn_task() {
        let handle = tokio::spawn(async { 42 });
        assert_eq!(handle.await.unwrap(), 42);
    }

    #[tokio::test]
    async fn test_select() {
        let fast = async { "fast" };
        let slow = async {
            tokio::time::sleep(Duration::from_millis(100)).await;
            "slow"
        };

        tokio::select! {
            result = fast => assert_eq!(result, "fast"),
            result = slow => panic!("should not reach here"),
        }
    }

    #[tokio::test]
    async fn test_timeout_success() {
        let op = async { "done" };
        let result = tokio::time::timeout(Duration::from_millis(100), op)
            .await
            .unwrap();
        assert_eq!(result, "done");
    }

    #[tokio::test]
    async fn test_timeout_exceeded() {
        let op = async {
            tokio::time::sleep(Duration::from_secs(10)).await;
            "done"
        };
        assert!(tokio::time::timeout(Duration::from_millis(10), op).await.is_err());
    }

    #[tokio::test]
    async fn test_async_channel() {
        let (tx, mut rx) = tokio::sync::mpsc::channel(32);

        tokio::spawn(async move {
            tx.send("hello").await.unwrap();
        });

        let msg = rx.recv().await.unwrap();
        assert_eq!(msg, "hello");
    }
}
