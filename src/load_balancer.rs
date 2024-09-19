use core::net::SocketAddr;
use std::io;
use tokio::net::{TcpListener, TcpStream};


pub async fn run_load_balancer(
	lb_addr: SocketAddr,
	server_addrs: &[SocketAddr],
	verbose: bool
) -> io::Result<()> {
	let listener = TcpListener::bind(&lb_addr).await?;
	if verbose { println!("[L] Listening on {}", lb_addr); }

	let mut next_server_index = 0;

	loop {
		// wait for client to connect
		let (socket, client_address) = match listener.accept().await {
			Err(e) => {
				if verbose { println!("[L] Failed to accept client. {:?}", e); }
				continue;
			}
			Ok((s, addr)) => {
				if verbose { println!("[L] New connection to {}", addr); }
				(s, addr)
			}
		};

		// select next server
		let server_address = server_addrs[next_server_index].clone();
		next_server_index = (next_server_index + 1) % server_addrs.len();

		// send client to selected server
		tokio::spawn(async move {
			if let Err(e) = handle_client(socket, client_address, server_address, verbose).await {
				if verbose {
					println!("[L] Failed to connect client {} to server {}. {:?}",
						 client_address,
						 server_address,
						 e
					);
				}
			}
		});
	}
}
async fn handle_client(
	mut client_socket: TcpStream,
	client_addr: SocketAddr,
	server_addr: SocketAddr,
	verbose: bool
) -> io::Result<(u64, u64)> {
	// connect
	let mut server_socket = TcpStream::connect(server_addr).await?;
	if verbose { println!("[L] Assigned client at {} to server at {}.", client_addr, server_addr); }

	// forward traffic
	tokio::io::copy_bidirectional(&mut client_socket, &mut server_socket).await
}
