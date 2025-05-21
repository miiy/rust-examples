use bytes::Bytes;
use tokio::sync::mpsc;

#[derive(Debug)]
enum Command {
    Get {
        key: String
    },
    Set {
        key: String,
        value: Bytes
    }
}

#[tokio::main]
async fn main() {
    let (tx, mut rx) = mpsc::channel(32);
    // The `Sender` handles are moved into the tasks. As there are two
    // tasks, we need a second `Sender`.
    let tx2 = tx.clone();

    // Spawn two tasks, one gets a key, the other sets a key
    let t1 = tokio::spawn(async move {
        let cmd = Command::Get {
            key: "key".to_string()
        };

        tx.send(cmd).await.unwrap();
    });


    let t2 = tokio::spawn(async move {
        let cmd = Command::Set {
            key: "foo".to_string(),
            value: "bar".into()
        };

        tx2.send(cmd).await.unwrap();
    });


    use mini_redis::client;
    // The `move` keyword is used to **move** ownership of `rx` into the task.
    let manager = tokio::spawn(async move {
        // Establish a connection to the server
        let mut client = client::connect("127.0.0.1:6379").await.unwrap();

        // Start receiving messages
        while let Some(cmd) = rx.recv().await {
            use Command::*;

            match cmd {
                Get { key }  => {
                    client.get(&key).await.unwrap();
                }
                Set { key, value } => {
                    client.set(&key, value).await.unwrap();
                }
            }
        }

    });

    t1.await.unwrap();
    t2.await.unwrap();
    manager.await.unwrap()
}