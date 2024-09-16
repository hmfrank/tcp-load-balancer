use std::env;
use std::io;
use tokio::io::AsyncWriteExt;
use tokio::net::{TcpListener, TcpStream};


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

    let listener = TcpListener::bind(&addr).await?;
    println!("[S] Listening on {}", addr);

    loop {
        match listener.accept().await {
            Ok((socket, client_address)) => {
                println!("[S] New connection to {}", client_address);

                let addr = addr.clone();
                tokio::spawn(async move {
                    if let Err(e) = handle_client(socket, &addr).await {
                        println!("[S] Failed to handle client at {}. {:?}", client_address, e);
                    }
                });
            }
            Err(e) => {
                println!("[S] Failed to accept client. {:?}", e);
            }
        };
    }
}

async fn handle_client(mut socket: TcpStream, listen_addr: &str) -> io::Result<()> {
    let msg = format!("Hello from {}.", listen_addr);
    socket.write_all(msg.into_bytes().as_slice()).await
}
