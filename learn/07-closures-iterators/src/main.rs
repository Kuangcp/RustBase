fn main() {
    closure_basics();
    iterator_basics();
    consumers();
    custom_iterator();
}

// ====== 闭包 ======

fn closure_basics() {
    println!("--- Closures ---");

    // 基本闭包
    let add = |a, b| a + b;
    println!("基本闭包: {}", add(3, 4) + add(1, 2));

    // 捕获外部变量
    let greeting = String::from("Hello");
    let name = String::from("Rust");
    let say_hello = || println!("捕获环境: {greeting}, {name}!");
    say_hello();

    // FnMut：可变捕获
    let mut counter = 0;
    let mut increment = || {
        counter += 1;
    };
    for _ in 0..5 {
        increment();
    }
    println!("可变闭包: counter = {counter}");
    println!();
}

// ====== 迭代器 ======

fn iterator_basics() {
    println!("--- Iterators ---");

    let numbers = vec![10, 20, 30, 40, 50];

    // map：转换
    let doubled: Vec<i32> = numbers.iter().map(|x| x * 2).collect();
    println!("map: {doubled:?}");

    // filter：过滤
    let small: Vec<&i32> = numbers.iter().filter(|&&x| x < 30).collect();
    println!("filter: {small:?}");

    // enumerate：带索引
    let indexed: Vec<(usize, &i32)> = numbers.iter().enumerate().collect();
    println!("enumerate: {indexed:?}");

    // zip：合并两个迭代器
    let names = vec![1, 2, 3];
    let values = vec![100, 200, 300];
    let pairs: Vec<_> = names.iter().zip(values.iter()).collect();
    println!("zip: {pairs:?}");
    println!();
}

// ====== 消费者 ======

fn consumers() {
    println!("--- Consumers ---");

    let numbers = vec![10, 20, 30, 40, 50];

    // sum
    let total: i32 = numbers.iter().sum();
    println!("sum: {total}");

    // any：是否存在满足条件的元素
    let has_large = numbers.iter().any(|&x| x > 100);
    println!("any > 100: {has_large}");

    // find：找到第一个满足条件的
    let first_big = numbers.iter().find(|&&x| x > 3);
    println!("find first > 3: {first_big:?}");

    // fold：累积折叠
    let product = numbers.iter().fold(1, |acc, &x| acc * x / 10);
    println!("fold: {product}");

    // 链式调用
    let result: i32 = numbers
        .iter()
        .filter(|&&x| x > 20)
        .map(|&x| x / 10)
        .sum();
    println!("filter -> map -> sum: {result}");
    println!();
}

// ====== 自定义迭代器 ======

struct Fibonacci {
    a: u64,
    b: u64,
}

impl Fibonacci {
    fn new() -> Self {
        Fibonacci { a: 1, b: 1 }
    }
}

impl Iterator for Fibonacci {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        let result = self.a;
        let new_b = self.a + self.b;
        self.a = self.b;
        self.b = new_b;
        Some(result)
    }
}

fn custom_iterator() {
    println!("--- Custom Iterator ---");

    let fibs: Vec<u64> = Fibonacci::new().take(10).collect();
    let display: Vec<String> = fibs.iter().map(|x| x.to_string()).collect();
    println!("Fibonacci: {}", display.join(" "));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_closure_add() {
        let add = |a, b| a + b;
        assert_eq!(add(2, 3), 5);
    }

    #[test]
    fn test_closure_mut() {
        let mut acc = 0;
        let mut add = || acc += 1;
        add();
        add();
        add();
        assert_eq!(acc, 3);
    }

    #[test]
    fn test_iterator_map() {
        let v = vec![1, 2, 3];
        let result: Vec<i32> = v.iter().map(|x| x * 2).collect();
        assert_eq!(result, vec![2, 4, 6]);
    }

    #[test]
    fn test_iterator_filter() {
        let v = vec![1, 2, 3, 4, 5];
        let result: Vec<&i32> = v.iter().filter(|&&x| x % 2 == 0).collect();
        assert_eq!(result, vec![&2, &4]);
    }

    #[test]
    fn test_fibonacci() {
        let fibs: Vec<u64> = Fibonacci::new().take(7).collect();
        assert_eq!(fibs, vec![1, 1, 2, 3, 5, 8, 13]);
    }

    #[test]
    fn test_fold() {
        let v = vec![1, 2, 3, 4, 5];
        let sum = v.iter().fold(0, |acc, &x| acc + x);
        assert_eq!(sum, 15);
    }
}
