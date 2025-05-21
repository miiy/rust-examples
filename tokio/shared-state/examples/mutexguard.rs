use std::sync::{Mutex, MutexGuard};

// async fn increment_and_do_stuff(mutex: &Mutex<i32>) {
//     let mut lock: MutexGuard<i32> = mutex.lock().unwrap();
//     *lock += 1;

//     do_something_async().await;
// } // lock goes out of scope here
// 
// 这是因为 std::sync::MutexGuard 类型不是 Send 。
// 这意味着你不能将互斥锁发送到另一个线程，而错误的发生是因为 Tokio 运行时可以在每个线程之间移动一个任务。
// 为了避免这种情况，您应该重构代码，使互斥锁的析构函数在 .await 之前运行。


// This works!
async fn increment_and_do_stuff(mutex: &Mutex<i32>) {
    {
        let mut lock: MutexGuard<i32> = mutex.lock().unwrap();
        *lock += 1;
    } // lock goes out of scope here

    do_something_async().await;
}

// // This fails too.
// async fn increment_and_do_stuff(mutex: &Mutex<i32>) {
//     let mut lock: MutexGuard<i32> = mutex.lock().unwrap();
//     *lock += 1;
//     drop(lock);

//     do_something_async().await;
// }

async fn do_something_async() {
    println!("Doing something async...");
}

#[tokio::main]
async fn main(){
    tokio::spawn(async move {
        let mutex = Mutex::new(0);
        increment_and_do_stuff(&mutex).await;
    });

}
