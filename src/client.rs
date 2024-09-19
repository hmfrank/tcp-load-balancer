use std::io;
use tokio::io::AsyncReadExt;
use tokio::net::{TcpStream};

pub async fn run_client(address: &str, verbose: bool) -> io::Result<()> {
	// connect to server
	let mut connection = TcpStream::connect(&address).await?;
	if verbose { println!("[C] Connected to {}", address); }

	// read stream
	let mut buffer = vec![0u8; 4086];
	let n = connection.read(buffer.as_mut_slice()).await?;
	let buffer: Vec<u8> = buffer.into_iter().take(n).collect();

	if verbose {
		println!("[C] Received {} bytes: {}",
				 n,
				 match String::from_utf8(buffer) {
					 Ok(msg) => format!("\"{}\"", msg),
					 Err(e) => format!("{:?}", e.into_bytes()),
				 }
		);
	}

	Ok(())
}