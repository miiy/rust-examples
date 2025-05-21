use tokio::io::{self, AsyncWriteExt};
use tokio::fs::File;

#[tokio::main]
async fn main() -> io::Result<()> {
    let mut file = File::create("foo1.txt").await?;

    // Writes some prefix of the byte string, but not necessarily all of it.
    file.write_all(b"some bytes").await?;

    Ok(())
}
