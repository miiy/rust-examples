use bytes::Bytes;
use tokio::sync::{mpsc, oneshot};

/// Multiple different commands are multiplexed over a single channel.
#[derive(Debug)]
enum Command {
    Get {
        key: String,
        resp: Responder<Option<Bytes>>
    },
    Set {
        key: String,
        value: Bytes,
        resp: Responder<()>
    }
}

/// Provided by the requester and used by the manager task to send
/// the command response back to the requester.
type Responder<T> = oneshot::Sender<mini_redis::Result<T>>;

#[tokio::main]
async fn main() {
    let (tx, mut rx) = mpsc::channel(32);
    // The `Sender` handles are moved into the tasks. As there are two
    // tasks, we need a second `Sender`.
    let tx2 = tx.clone();

    // Spawn two tasks, one gets a key, the other sets a key
    let t1 = tokio::spawn(async move {
        let (resp_tx, resp_rx) = oneshot::channel();
        let cmd = Command::Get {
            key: "key".to_string(),
            resp: resp_tx
        };

        // Send the GET request
        tx.send(cmd).await.unwrap();
        
        // Wait for the response
        let res = resp_rx.await;
        println!("GOT = {:?}", res);
    });


    let t2 = tokio::spawn(async move {
        let (rexp_tx, rexp_rx) = oneshot::channel();
        let cmd = Command::Set {
            key: "foo".to_string(),
            value: "bar".into(),
            resp: rexp_tx,
        };

        // Send the SET request
        tx2.send(cmd).await.unwrap();

        // Await for the response
        let res = rexp_rx.await;
        println!("GOT = {:?}", res);
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
                Get { key, resp }  => {
                    let res = client.get(&key).await;
                    // Ignore errors
                    let _ = resp.send(res);
                }
                Set { key, value, resp} => {
                    let res = client.set(&key, value).await;
                    // Ignore errors
                    let _ = resp.send(res);
                }
            }
        }

    });

    t1.await.unwrap();
    t2.await.unwrap();
    manager.await.unwrap()
}