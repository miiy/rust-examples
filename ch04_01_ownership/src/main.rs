// 什么是所有权？
// Rust 的核心功能（之一）就是所有权（ownership）
// 所有程序都必须管理其运行时使用计算机内存的方式。
//    一些语言中具有垃圾回收机制，在程序运行时有规律地寻找不再使用的内存
//    程序员必须亲自分配和释放内存
//    通过所有权系统管理内存
//
// 所有权规则
//     1. Rust 中的每一个值都有一个 所有者（owner）。
//     2. 值在任一时刻有且只有一个所有者。
//     3. 当所有者（变量）离开作用域，这个值将被丢弃。

fn main() {
    let mut s = String::from("hello");
    s.push_str(", world!");

    println!("{}", s);

    let s = String::from("hello");

    takes_ownership(s);

    let x = 5;

    makes_copy(x);

}

// 作用域（scope）是一个项（item）在程序中有效的范围
fn variable_scope() {  // s 在这里无效，它尚未声明
    let s = "hello";   // 从此处起，s 是有效的

    // 使用 s
}                      // 此作用域已结束，s 不再有效

// 当 s 进入作用域 时，它就是有效的。
// 这一直持续到它 离开作用域 为止。
//
//
fn string() {

}

fn takes_ownership(some_thing: String) {
    println!("{}", some_thing);
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}

fn gives_ownership() -> String {
    let some_thing = String::from("yours");
    some_thing
}
