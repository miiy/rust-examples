// 使用 Box<T> 指向堆上的数据
#![allow(dead_code)]
#![allow(unused_variables)]

use crate::List::{Cons, Nil};

fn main() {
    // 使用 box 在堆上存储一个 i32 值
    box_example();

    // 使用 List 枚举存储列表 1, 2, 3
    list_enum();

    // 为了拥有已知大小而使用 Box<T> 的 List 定义
    list_enum2();
}

// 使用 box 在堆上存储一个 i32 值
fn box_example() {
    let b = Box::new(5);
    println!("b = {b}");
}

// Box 允许创建地推类型
// cons list 的更多内容

// 第一次尝试定义一个代表 i32 值的 cons list 数据结构的枚举
// enum List {
//     Cons(i32, List),
//     Nil,
// }

// 使用 List 枚举存储列表 1, 2, 3
fn list_enum() {
    // let list = Cons(1, Cons(2, Cons(3, Nil)));
}

// 计算非递归类型的大小

// 使用 Box<T> 给递归类型一个已知的大小

enum List {
    Cons(i32, Box<List>),
    Nil,
}

// 为了拥有已知大小而使用 Box<T> 的 List 定义
fn list_enum2() {
    let list2 = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
}
