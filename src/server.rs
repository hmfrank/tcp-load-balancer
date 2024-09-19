use std::io;
use tokio::io::AsyncWriteExt;
use tokio::net::{TcpListener, TcpStream};

pub async fn run_server(address: &str, verbose: bool) -> io::Result<()> {
	let listener = TcpListener::bind(&address).await?;
	if verbose { println!("[S] Listening on {}", address); }

	loop {
		let (socket, client_address) = match listener.accept().await {
			Err(e) => {
				if verbose { println!("[S] Failed to accept client. {:?}", e); }
				continue;
			}
			Ok((s, addr)) => {
				if verbose { println!("[S] New connection to {}", addr); }
				(s, addr)
			}
		};

		let address = address.to_string();
		tokio::spawn(async move {
			if let Err(e) = handle_client(socket, &address).await {
				if verbose {
					println!("[S] Failed to handle client at {}. {:?}", client_address, e);
				}
			}
		});
	}
}

async fn handle_client(mut socket: TcpStream, listen_addr: &str) -> io::Result<()> {
	let msg = format!("Hello from {}.", listen_addr);
	socket.write_all(msg.into_bytes().as_slice()).await
}
