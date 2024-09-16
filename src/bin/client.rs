use std::{env, io};
use tokio::io::AsyncReadExt;
use tokio::net::{TcpStream};

#[tokio::main]
async fn main() -> io::Result<()> {
    let addr = {
        let args: Vec<_> = env::args().collect();

        if args.len() < 2 {
            "127.0.0.1:8080".to_string()
        } else {
            args.into_iter().skip(1).next().unwrap()
        }
    };

    let mut connection = TcpStream::connect(&addr).await?;
    println!("[C] Connected to {}", addr);

    let mut buffer = vec![0u8; 4086];
    let n = connection.read(buffer.as_mut_slice()).await?;

    let buffer: Vec<u8> = buffer.into_iter().take(n).collect();
    println!("[C] Received {} bytes: {}",
        n,
        match String::from_utf8(buffer) {
            Ok(msg) => format!("\"{}\"", msg),
            Err(e) => format!("{:?}", e.into_bytes()),
        }
    );

    Ok(())
}
