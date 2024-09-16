use core::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::io;
use tokio::net::{TcpListener, TcpStream};


#[tokio::main]
async fn main() -> io::Result<()> {
    let servers = vec![
        SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8081),
        SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8082),
        SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8083),
        SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8084),
    ];
    let mut next_server_index = 0;

    // TCP listener, der neue Anfragen aufnimmt
    let listener = TcpListener::bind("127.0.0.1:8080").await?;

    loop {
        match listener.accept().await {
            Ok((socket, client_address)) => {
                let server_address = servers[next_server_index].clone();
                next_server_index = (next_server_index + 1) % servers.len();

                tokio::spawn(async move {
                    if let Err(e) = handle_client(socket, client_address, server_address).await {
                        println!("Failed to connect client {:?} to server {:?}. {:?}", client_address, server_address, e);
                    }
                });
            }
            Err(e) => {
                println!("Failed to accept client. {:?}", e);
            }
        };
    }
}

async fn handle_client(mut client_socket: TcpStream, client_addr: SocketAddr, server_addr: SocketAddr) -> io::Result<(u64, u64)> {
    // connection mit server aufbauen
    let mut server_socket = TcpStream::connect(server_addr).await?;
    println!("Connected client at {:?} to server at {:?}.", client_addr, server_addr);

    // per copy verbinden
    tokio::io::copy_bidirectional(&mut client_socket, &mut server_socket).await
}
