// 使用 Drop Trait 运行清理代码
#![allow(unused_variables)]

use std::mem::drop;

struct CustomSmartPointer {
    data: String
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

fn main() {
    // 结构体 CustomSmartPointer，其实现了放置清理代码的 Drop trait
    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };
    let d = CustomSmartPointer {
        data: String::from("other stuff"),
    };
    println!("CustomSmartPointers created.");

    // 尝试手动调用 Drop trait 的 drop 方法提早清理
    let c = CustomSmartPointer {
        data: String::from("some data"),
    };
    println!("CustomSmartPointers created.");
    // c.drop();
    drop(c);
    println!("CustomSmartPointers dropped before the end of main.");
}
