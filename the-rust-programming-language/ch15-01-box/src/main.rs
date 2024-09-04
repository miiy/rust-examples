// enum List {
//     Cons(i32, List),
//     Nil,
// }

#[allow(dead_code)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}

use crate::List::{Cons, Nil};

#[allow(unused_variables)]
fn main() {
    box_example();

    // let list = Cons(1, Cons(2, Cons(3, Nil)))
    let list2 = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
}

// 使用 box 在堆上存储一个 i32 值
fn box_example() {
    let b = Box::new(5);
    println!("b = {b}");
}