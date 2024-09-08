use std::env;
use std::io;
use tokio::net::{TcpListener};


#[tokio::main]
async fn main() -> io::Result<()> {
    let addr = env::args().skip(1).next().unwrap();

    // TCP listener, der neue Anfragen aufnimmt
    let listener = TcpListener::bind(addr).await?;

    loop {
        let _ = listener.accept().await;
    }
}
