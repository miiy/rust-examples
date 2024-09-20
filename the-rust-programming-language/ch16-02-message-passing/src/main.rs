// 使用消息在线程间传送数据
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    // 创建一个信道，并将其两端复制给 tx 和 rx
    create_channel();

    // 将 tx 移动到一个新建的线程中并发送 "hi"
    move_tx();
    println!();

    // 在主线程中接受并打印内容 "hi"
    print_hi();
    println!();

    // 信道与所有权转移
    channel_ownership();
    println!();

    // 发送多个值并观察接收者的等待
    send_multi_val();
    println!();

    // 从多个生产者发送多个消息
    multi_sender();
}

// 创建一个信道，并将其两端复制给 tx 和 rx
fn create_channel() {
    // let (tx, rx) = mpsc::channel();
}

// 将 tx 移动到一个新建的线程中并发送 "hi"
fn move_tx() {
    // let (tx, rx) = mpsc::channel();

    // thread::spawn(move || {
    //     let val = String::from("hi");
    //     tx.send(val).unwrap();
    // })
}

// 在主线程中接受并打印内容 "hi"
fn print_hi() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
    });

    let received = rx.recv().unwrap();
    println!("Got: {received}");
}

// 信道与所有权转移
// 在我们已经发送到信道中后，尝试使用 val 引用
fn channel_ownership() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
        // println!("val is {val}");
    });

    let received = rx.recv().unwrap();
    println!("Got: {received}");
}

// 发送多个值并观察接收者的等待
// 发送多个消息，并在每次发送后暂停一段时间
fn send_multi_val() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {received}");
    }
}

// 通过克隆发送者来创建多个生产者
// 从多个生产者发送多个消息
fn multi_sender() {
    let (tx, rx) = mpsc::channel();

    let tx1 = tx.clone();
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {received}");
    }
}
