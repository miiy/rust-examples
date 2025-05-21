use tokio::sync::Mutex;

async fn increment_and_do_stuff(mutex: &Mutex<i32>) {
    let mut lock = mutex.lock().await;
    *lock += 1;

    do_something_async().await;
}

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
