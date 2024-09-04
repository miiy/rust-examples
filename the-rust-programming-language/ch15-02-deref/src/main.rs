#![allow(unused_variables)]
fn main() {
    deref_example();
    box_deref();
    mybox_example();
}

// 使用解引用运算符来跟踪 i32 值的引用
fn deref_example() {
    let x = 5;
    let y = &x;

    assert_eq!(5, x);
    assert_eq!(5, *y);
}

// 在 Box<i32> 上使用解引用运算符
fn box_deref() {
    let x = 5;
    let y = Box::new(5);
    assert_eq!(5, x);
    assert_eq!(5, *y);
}

// 定义智能指针
// 定义 MyBox<T> 类型
struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

fn mybox_example() {
    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    // assert_eq!(5, *y);
}