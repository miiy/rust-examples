#![allow(unused_variables)]
#![allow(dead_code)]

enum List {
    Cons(i32, Box<List>),
    Nil,
}

use crate::List::{Cons, Nil};

fn main() {
    // 展示不能用两个 Box<T> 的列表尝试共享第三个列表的所有权
    let a = Cons(5, Box::new(Cons(10, Box::new(Nil))));
    let b = Cons(3, Box::new(a));
    // let c = Cons(4, Box::new(a));
}
