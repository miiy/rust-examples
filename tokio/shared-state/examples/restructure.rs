use std::sync::Mutex;

struct CanIncrement {
    mutex: Mutex<i32>,
}

impl CanIncrement {
    fn new() -> Self {
        Self {
            mutex: Mutex::new(0),
        }
    }

    fn increment(&self) {
       let mut lock = self.mutex.lock().unwrap();
       *lock += 1;
    }
}

async fn increment_and_do_stuff(can_incr: &CanIncrement) {
    can_incr.increment();
    do_something_async().await;
}

async fn do_something_async() {
    println!("Doing something async...");
}

#[tokio::main]
async fn main(){
    tokio::spawn(async move {
        let can_incr = CanIncrement::new();
        increment_and_do_stuff(&can_incr).await;
    });

}
