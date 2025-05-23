use tokio::io;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:6142").await?;

    loop {
        let (mut socket, _) = listener.accept().await?;

        tokio::spawn(async move {
            // Copy data here
            let (mut re, mut wr) = socket.split();

            if io::copy(&mut re, &mut wr).await.is_err() {
                eprintln!("Failed to copy data");
            }
        });
    }
}