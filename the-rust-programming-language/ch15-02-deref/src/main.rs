// 通过 Deref trait 将智能指针当做常规引用处理
#![allow(unused_variables)]

use std::ops::Deref;

fn main() {
    // 追踪指针的值
    // 使用解引用运算符来跟踪 i32 值的引用
    deref_example();

    // 向引用一样使用 Box<T>
    // 在 Box<i32> 上使用解引用运算符
    box_deref();

    // 定义智能指针
    mybox_example();

    // 通过实现 Deref trait 将某些类型像引用一样处理
    // MyBox<T> 上的 Deref 实现
    mybox_example2();

    // 函数和方法的隐式 Deref 强制转换
    deref_convert();

    deref_convert2();
}

// 追踪指针的值
// 使用解引用运算符来跟踪 i32 值的引用
fn deref_example() {
    let x = 5;
    let y = &x;

    assert_eq!(5, x);
    assert_eq!(5, *y);
}

// 向引用一样使用 Box<T>
// 在 Box<i32> 上使用解引用运算符
fn box_deref() {
    let x = 5;
    let y = Box::new(5);
    assert_eq!(5, x);
    assert_eq!(5, *y);
}

// 定义智能指针

// 尝试以使用引用和 Box<T> 相同的方式使用 MyBox<T>
fn mybox_example() {
    // 定义 MyBox<T> 类型
    struct MyBox<T>(T);

    impl<T> MyBox<T> {
        fn new(x: T) -> MyBox<T> {
            MyBox(x)
        }
    }

    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    // assert_eq!(5, *y);
}

// 通过实现 Deref trait 将某些类型像引用一样处理
// MyBox<T> 上的 Deref 实现
fn mybox_example2() {
    struct MyBox<T>(T);

    impl<T> MyBox<T> {
        fn new(x: T) -> MyBox<T> {
            MyBox(x)
        }
    }

    impl<T> Deref for MyBox<T> {
        type Target = T;

        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }

    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);
}

// 函数和方法的隐式 Deref 强制转换

// hello 函数有着 &str 类型的参数 name
fn hello(name: &str) {
    println!("Hello, {name}!")
}

// 因为 Deref 强制转换，使⽤ MyBox<String> 的引⽤调⽤ hello 是可⾏的
fn deref_convert() {
    struct MyBox<T>(T);

    impl<T> MyBox<T> {
        fn new(x: T) -> MyBox<T> {
            MyBox(x)
        }
    }
    impl<T> Deref for MyBox<T> {
        type Target = T;

        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }

    let m = MyBox::new(String::from("Rust"));
    hello(&m);
}

// 如果 Rust 没有 Deref 强制转换则必须编写的代码
fn deref_convert2() {
    struct MyBox<T>(T);

    impl<T> MyBox<T> {
        fn new(x: T) -> MyBox<T> {
            MyBox(x)
        }
    }
    impl<T> Deref for MyBox<T> {
        type Target = T;

        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }

    let m = MyBox::new(String::from("Rust"));
    hello(&(*m)[..]);
}

// Deref 强制转换如何与可变性交互
