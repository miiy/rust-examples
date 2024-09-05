// 捕获引用或者移动所有权
use std::thread;

fn main() {
    borrows();
    println!();

    borrows_mut();
    println!();

    move_thread();
}

// 定义并调用一个捕获不可变引用的闭包
fn borrows() {
    let list = vec![1, 2, 3];
    println!("Before defining closure: {list:?}");

    let only_borrows = || println!("From closure: {list:?}");

    println!("Before calling closure: {list:?}");
    only_borrows();
    println!("After calling closure: {list:?}");
}

// 定义并调用一个捕获可变引用的闭包
fn borrows_mut() {
    let mut list = vec![1, 2, 3];
    println!("Before defining closure: {list:?}");

    let mut borrows_mutably = || list.push(7);

    borrows_mutably();
    println!("After calling closure: {list:?}");
}

// 使用 move 来强制闭包为线程获取 list 的所有权
fn move_thread() {
    let list = vec![1, 2, 3];
    println!("Before defining closure: {list:?}");

    thread::spawn(move || println!("From thread: {list:?}"))
        .join()
        .unwrap();
}