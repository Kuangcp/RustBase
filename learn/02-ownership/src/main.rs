fn main() {
    ownership_move();
    ownership_copy();
    borrowing();
    borrowing_mut();
    lifetime_basics();
}

/// 所有权移动：String 赋值后原变量失效
fn ownership_move() {
    println!("--- Ownership Move ---");

    let s1 = String::from("hello");
    let s2 = s1; // s1 的所有权移动到 s2

    // println!("{s1}"); // 编译错误：value borrowed here after move
    println!("s2 = {s2}");
    println!();
}

/// Copy trait：栈上的简单类型自动复制
fn ownership_copy() {
    println!("--- Ownership Copy ---");

    let x = 42;
    let y = x; // i32 实现了 Copy，y 是 x 的副本

    println!("x = {x}, y = {y}"); // 两个都能用
    println!();
}

/// 不可变借用：可以同时存在多个
fn borrowing() {
    println!("--- Borrowing (immutable) ---");

    let s = String::from("hello");
    let len = calculate_length(&s); // 传入借用，不转移所有权

    println!("'{s}' 的长度是 {len}");
    println!();
}

fn calculate_length(s: &str) -> usize {
    s.len()
    // s 离开作用域时，因为不拥有所有权，什么也不会发生
}

/// 可变借用：同一时间只能有一个
fn borrowing_mut() {
    println!("--- Borrowing (mutable) ---");

    let mut s = String::from("hello");
    change(&mut s);
    println!("修改后: {s}");
    println!();
}

fn change(s: &mut String) {
    s.push_str(", world");
}

/// 生命周期基础：函数签名中的 'a
fn lifetime_basics() {
    println!("--- Lifetime Basics ---");

    let string1 = String::from("long string");
    let result;
    {
        let string2 = String::from("xyz");
        result = longest(string1.as_str(), string2.as_str());
        println!("较长的字符串: {result}");
    }
    // result 在这里已经不能用了，因为 string2 已经离开作用域
}

/// 返回值的生命周期 = 两个参数中较短的那个
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() >= y.len() {
        x
    } else {
        y
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_length() {
        let s = String::from("hello");
        assert_eq!(calculate_length(&s), 5);
    }

    #[test]
    fn test_change() {
        let mut s = String::from("hello");
        change(&mut s);
        assert_eq!(s, "hello, world");
    }

    #[test]
    fn test_longest() {
        let s1 = String::from("long string");
        let s2 = String::from("xyz");
        assert_eq!(longest(&s1, &s2), "long string");
    }
}
