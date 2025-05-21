use mini_redis::client;

// 这无法编译，因为这两个任务都需要以某种方式访问 client 。
// 由于 Client 没有实现 Copy，如果没有一些代码来促进这种共享，它将无法编译。
// 此外，Client::set 需要 &mut self，这意味着调用它需要独占访问。我们可以为每个任务打开一个连接，但这并不理想。
// 我们不能使用 std::sync::Mutex，因为需要在保持锁的情况下调用 .await 。
// 我们可以使用 tokio::sync::Mutex，但这只允许一个 single in-flight 请求。如果客户端实现了 pipelining，异步互斥会导致连接利用不足。

#[tokio::main]
async fn main() {
    // Establish a connection to the server
    let mut client = client::connect("127.0.0.1:6379").await.unwrap();

    // Spawn two tasks, one gets a key, the other sets a key
    let t1 = tokio::spawn(async {
        let res = client.get("foo").await;
    });

    let t2 = tokio::spawn(async {
        client.set("foo", "bar".into()).await;
    });

    t1.await.unwrap();
    t2.await.unwrap();
}
