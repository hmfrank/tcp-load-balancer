use std::io;
use tokio::net::{TcpStream};

#[tokio::main]
async fn main() -> io::Result<()> {
    let connection = TcpStream::connect("127.0.0.1:8080").await?;

    Ok(())
}
