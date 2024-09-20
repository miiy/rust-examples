// 共享状态并发
#![allow(unused_imports)]

use std::thread;
use std::rc::Rc;
use std::sync::{Arc, Mutex};

fn main() {
    // Mutex<T> 的 API
    mutex_api();
    println!();

    // 在线程间共享 Mutex<T>
    // thread_mutex();

    // 多线程和多所有权
    // multi_thread_ownership();

    // 原子引用计数 Arc<T>
    arc();
}

// 互斥器一次只允许一个线程访问数据
// Mutex<T> 的 API
// 出于简单的考虑，在一个单线程上下文中探索 Mutex<T> 的 API
fn mutex_api() {
    let m = Mutex::new(5);

    {
        let mut num = m.lock().unwrap();
        *num = 6;
    }

    println!("m = {m:?}");
}

// 在线程间共享 Mutex<T>
// 程序启动了 10 个线程，每个线程都通过 Mutex<T> 来增加计数器的值
// fn thread_mutex() {
//     let counter = Mutex::new(0);
//     let mut handles = vec![];

//     for _ in 0..10 {
//         let handle = thread::spawn(move || {
//             let mut num = counter.lock().unwrap();

//             *num += 1;
//         });
//         handles.push(handle);
//     }

//     for handle in handles {
//         handle.join().unwrap();
//     }

//     println!("Result: {}", *counter.lock().unwrap());
// }

// 多线程和多所有权
// fn multi_thread_ownership() {
//     let counter = Rc::new(Mutex::new(0));
//     let mut handles = vec![];

//     for _ in 0..10 {
//         let counter = Rc::clone(&counter);
//         let handle = thread::spawn(move || {
//             let mut num = counter.lock().unwrap();

//             *num += 1;
//         });
//         handles.push(handle);
//     }

//     for handle in handles {
//         handle.join().unwrap();
//     }

//     println!("Result: {}", *counter.lock().unwrap());
// }

// 原子引用计数 Arc<T>
// 使⽤ Arc<T> 包装⼀个 Mutex<T> 能够实现在多线程之间共享所有权
fn arc() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}