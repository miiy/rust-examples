use std::thread;
use std::time::Duration;

fn main() {
    thread_example1();
    println!();

    thread_example2();
    println!();

    thread_example3();
    println!();

    thread_example4();
}

// 使用 spawn 创建新线程
// 创建一个打印某些内容的新线程，但是主线程打印其它内容
fn thread_example1() {
    thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {i} from the spawned thread!");
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {i} from the main thread!");
        thread::sleep(Duration::from_millis(1));
    }
}


// 使用 join 等待所有线程结束
// 从 thread::spawn 保存一个 JoinHandle 以确保该线程能够运行至结束
fn thread_example2() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {i} from the spawned thread!");
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {i} from the main thread!");
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap();
}

fn thread_example3() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {i} from the spawned thread!");
            thread::sleep(Duration::from_millis(1));
        }
    });

    handle.join().unwrap();

    for i in 1..5 {
        println!("hi number {i} from the main thread!");
        thread::sleep(Duration::from_millis(1));
    }
}

// 将 move 闭包与线程一同使用
// 使用 move 关键字强制获取它使用的值的所有权
fn thread_example4() {
    let v = vec![1, 2, 3];

    // let handle = thread::spawn(|| {
    //     println!("Here's vector: {v:?}");
    // });

    let handle = thread::spawn( move || {
        println!("Here's vector: {v:?}");
    });

    handle.join().unwrap();
}