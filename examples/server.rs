use std::env;
use std::io;
use tokio::net::{TcpListener, TcpStream};


#[tokio::main]
async fn main() -> io::Result<()> {
    let addr = env::args().skip(1).next().unwrap();

    // TCP listener, der neue Anfragen aufnimmt
    let listener = TcpListener::bind(addr).await?;

    loop {
        match listener.accept().await {
            Ok((socket, client_address)) => {
                println!("New connection to {:?}", client_address);

                tokio::spawn(async move {
                    if let Err(e) = echo(socket).await {
                        println!("Failed to echo client {:?}. {:?}", client_address, e);
                    }
                });
            }
            Err(e) => {
                println!("Failed to accept client. {:?}", e);
            }
        };
    }
}

async fn echo(mut socket: TcpStream) -> io::Result<()> {
    let (mut rx, mut tx) = socket.split();
    tokio::io::copy(&mut rx, &mut tx).await?;
    Ok(())
}
