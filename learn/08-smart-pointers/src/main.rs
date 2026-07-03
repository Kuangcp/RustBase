use std::cell::RefCell;
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    box_demo();
    rc_demo();
    refcell_demo();
    arc_mutex_demo();
}

// ====== Box<T>：堆分配 + 递归类型 ======

#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}

impl List {
    fn to_string(&self) -> String {
        match self {
            List::Cons(val, next) => format!("{val} -> {}", next.to_string()),
            List::Nil => String::from("Nil"),
        }
    }
}

fn box_demo() {
    println!("--- Box ---");

    // 用 Box 构造递归链表（不用 Box 会无限大小）
    let list = List::Cons(
        1,
        Box::new(List::Cons(
            2,
            Box::new(List::Cons(3, Box::new(List::Nil))),
        )),
    );

    println!("链表: {}", list.to_string());
    println!();
}

// ====== Rc<T>：引用计数共享所有权 ======

use std::rc::Rc;

fn rc_demo() {
    println!("--- Rc ---");

    let a = Rc::new(String::from("shared data"));
    println!("引用计数: {}", Rc::strong_count(&a));

    let b = Rc::clone(&a);
    println!("克隆后计数: {}", Rc::strong_count(&a));

    let c = Rc::clone(&a);
    println!("再克隆: {}", Rc::strong_count(&a));

    drop(c);
    println!("释放一个后: {}", Rc::strong_count(&a));

    // 两个引用指向同一数据
    assert_eq!(*a, *b);
    println!();
}

// ====== RefCell<T>：内部可变性 ======

fn refcell_demo() {
    println!("--- RefCell ---");

    let data = RefCell::new(10);

    // 不可变借用读取
    println!("值: {}", data.borrow());

    // 可变借用修改（运行时检查）
    *data.borrow_mut() += 20;
    println!("修改后: {}", data.borrow());

    // RefCell 在违反借用规则时 panic
    // let r1 = data.borrow();
    // let r2 = data.borrow_mut(); // panic! 已有不可变借用
}

// ====== Arc<T> + Mutex<T>：多线程共享可变状态 ======

fn arc_mutex_demo() {
    println!("--- Arc + Mutex ---");

    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for i in 0..3 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
            println!("线程 {i}: count = {num}");
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("最终 count = {}", *counter.lock().unwrap());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_box_list() {
        let list = List::Cons(
            1,
            Box::new(List::Cons(2, Box::new(List::Nil))),
        );
        assert_eq!(list.to_string(), "1 -> 2 -> Nil");
    }

    #[test]
    fn test_box_nil() {
        let list = List::Nil;
        assert_eq!(list.to_string(), "Nil");
    }

    #[test]
    fn test_rc_ref_count() {
        let a = Rc::new(42);
        assert_eq!(Rc::strong_count(&a), 1);

        let b = Rc::clone(&a);
        assert_eq!(Rc::strong_count(&a), 2);

        drop(b);
        assert_eq!(Rc::strong_count(&a), 1);
    }

    #[test]
    fn test_refcell_borrow() {
        let data = RefCell::new(5);
        assert_eq!(*data.borrow(), 5);

        *data.borrow_mut() += 10;
        assert_eq!(*data.borrow(), 15);
    }

    #[test]
    fn test_arc_mutex() {
        let counter = Arc::new(Mutex::new(0));
        let mut handles = vec![];

        for _ in 0..10 {
            let c = Arc::clone(&counter);
            handles.push(thread::spawn(move || {
                *c.lock().unwrap() += 1;
            }));
        }

        for h in handles {
            h.join().unwrap();
        }

        assert_eq!(*counter.lock().unwrap(), 10);
    }
}
