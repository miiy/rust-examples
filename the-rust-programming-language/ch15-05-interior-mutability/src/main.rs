// RefCell<T> 和内部可变性模式



fn main() {
    // 通过 RefCell<T> 在运行时检查借用规则

    // 内部可变性：不可变值的可变借用
    mut_example();

    // 内部可变性的用例：mock 对象

    // 使⽤ Rc<RefCell<i32>> 创建可以修改的 List
    rc_refcell();
}

// 内部可变性：不可变值的可变借用
fn mut_example() {
    // let x = 5;
    // let y = &mut x;
}

// 结合 Rc<T> 和 RefCell<T> 来拥有多个可变数据所有者
// 使⽤ Rc<RefCell<i32>> 创建可以修改的 List
#[allow(dead_code)]
#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}

use crate::List::{Cons, Nil};
use std::cell::RefCell;
use std::rc::Rc;

fn rc_refcell() {
    let value = Rc::new(RefCell::new(5));

    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));

    let b = Rc::new(Cons(Rc::new(RefCell::new(3)), Rc::clone(&a)));
    let c = Rc::new(Cons(Rc::new(RefCell::new(4)), Rc::clone(&a)));

    *value.borrow_mut() += 10;

    println!("a after = {a:?}");
    println!("b after = {b:?}");
    println!("c after = {c:?}");
}
