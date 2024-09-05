// Rc<T> 引用计数智能指针
#![allow(unused_variables)]
#![allow(dead_code)]

enum List {
    Cons(i32, Box<List>),
    Nil,
}

fn main() {
    // 展示不能用两个 Box<T> 的列表尝试共享第三个列表的所有权
    example1();

    // 使用 Rc<T> 定义的 List
    example2();

    // 克隆 RC<T> 会增加引用计数
    example3();
}

// 展示不能用两个 Box<T> 的列表尝试共享第三个列表的所有权
fn example1() {
    use crate::List::{Cons, Nil};

    let a = Cons(5, Box::new(Cons(10, Box::new(Nil))));
    let b = Cons(3, Box::new(a));
    // let c = Cons(4, Box::new(a));
}

// 使用 Rc<T> 定义的 List
use std::rc::Rc;
enum RCList {
    Cons(i32, Rc<RCList>),
    Nil,
}

fn example2() {
    use crate::RCList::{Cons, Nil};

    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    let b = Cons(3, Rc::clone(&a));
    let c = Cons(4, Rc::clone(&a));
}

// 克隆 RC<T> 会增加引用计数
// 打印出引用计数
fn example3() {
    use crate::RCList::{Cons, Nil};

    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("count after creating a = {}", Rc::strong_count(&a));
    let b = Cons(3, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a));
    {
        let c = Cons(4, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a));
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));
}