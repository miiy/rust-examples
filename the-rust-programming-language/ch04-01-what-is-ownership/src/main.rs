// 什么是所有权？
// Rust 的核心功能（之一）就是所有权（ownership）
// 所有程序都必须管理其运行时使用计算机内存的方式。
//    一些语言中具有垃圾回收机制，在程序运行时有规律地寻找不再使用的内存
//    程序员必须亲自分配和释放内存
//    通过所有权系统管理内存
#![allow(unused_variables)]

fn main() {
    // 所有权规则
    //
    // 1. Rust 中的每一个值都有一个 所有者（owner）。
    // 2. 值在任一时刻有且只有一个所有者。
    // 3. 当所有者（变量）离开作用域，这个值将被丢弃。

    // 变量作用域
    variable_scope();

    // String 类型
    string_example();

    // 内存与分配
    memory_alloc();

    // 变量与数据交互的方式（一）：移动
    variable_move();

    // 变量与数据交互的方式（二）：克隆
    variable_clone();

    // 只在栈上的数据：拷贝
    stack_copy();

    // 所有权与函数
    ownership_fn();

    // 返回值与作用域
    result_scope();

}


// 变量作用域
// 作用域（scope）是一个项（item）在程序中有效的范围
// 一个变量和其有效的作用域
fn variable_scope() {  // s 在这里无效，它尚未声明
    let s: &str = "hello";   // 从此处起，s 是有效的

    // 使用 s
}                      // 此作用域已结束，s 不再有效

// 当 s 进入作用域 时，它就是有效的。
// 这一直持续到它 离开作用域 为止。


// String 类型
fn string_example() {
    // 使用 from 函数基于字符串字面值来创建 String
    let s = String::from("hello");

    // 可以修改此类字符串
    let mut s = String::from("hello");

    s.push_str(", world!"); // push_str() 在字符串后追加字面值

    println!("{}", s); // 将打印 `hello, world!`
}


// 内存与分配
fn memory_alloc() {
    {
        let s = String::from("hello"); // 从此处起，s 是有效的

        // 使用 s
    } // 此作用域已结束，
      // s 不再有效
}

// 变量与数据交互的方式（一）：移动
fn variable_move() {
    // 将变量 x 的证书值赋给 y
    let x = 5;
    let y = x;

    // String 版本
    let s1 = String::from("hello");
    let s2 = s1;

    // String 由三部分组成：
    // -------------------
    // |name     | value |
    // |ptr      |       |
    // |len      | 5     |
    // |capacity | 5     |
    // -------------------
}

// 变量与数据交互的方式（二）：克隆
fn variable_clone() {
    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {}, s2 = {}", s1, s2)
}

// 只在栈上的数据：拷贝
fn stack_copy() {
    let x = 5;
    let y = x;

    println!("x = {}, y = {}", x, y)
}

// 所有权与函数
fn ownership_fn() {
    let s = String::from("hello"); // s 进入作用域
    takes_ownership(s);         // s 的值移动到函数里 ...
                                            // ... 所以到这里不再有效

    let x = 5;                        // x 进入作用域

    makes_copy(x);            // x 应该移动函数里，但 i32 是 Copy 的，所以在后面可继续使用 x
} // 这里， x 先移出了作用域，然后是 s。但因为 s 的值已被移走，
  // 没有特殊之处

fn takes_ownership(some_thing: String) { // some_thing 进入作用域
    println!("{}", some_thing);
} // 这里，some_string 移出作用域并调用 `drop` 方法。
  // 占用的内存被释放

fn makes_copy(some_integer: i32) { // some_integer 进入作用域
    println!("{}", some_integer);
} // 这里，some_integer 移出作用域。没有特殊之处

// 返回值与作用域
fn result_scope() {
    let s1 = gives_ownership(); // gives_ownership 将返回值转移给 s1

    let s2 = String::from("hello"); // s2 进入作用域

    let s3 = takes_and_gives_back(s2); // s2 被移动到 takes_and_gives_back 中，它也将返回值移给 s3
} // 这里，s3 移出作用域并被丢弃。s2 也移出作用域，但已被移走，所以什么也不会发生。s1 离开作用域并被丢弃

fn gives_ownership() -> String { // gives_ownership 会将返回值移动给调用它的函数
    let some_thing = String::from("yours"); // some_thing 进入作用域
    some_thing // 返回 some_thing 并移出给调用的函数
}

// takes_and_gives_back 将传入字符串并返回该值
fn takes_and_gives_back(a_string: String) -> String { // a_string 进入作用域
    a_string // 返回 a_string 并移出给调用的函数
}