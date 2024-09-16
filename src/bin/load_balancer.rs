use core::net::SocketAddr;
use std::{env, io};
use tokio::net::{TcpListener, TcpStream};

#[tokio::main]
async fn main() -> io::Result<()> {
    let (lb_addr, server_addrs) = {
        let mut addrs = env::args()
            .skip(1)
            .map(|addr| addr.parse::<SocketAddr>())
            .filter(|result| result.is_ok() )
            .map(|result| result.unwrap());

        (addrs.next(), addrs.collect::<Vec<SocketAddr>>())
    };

    if lb_addr.is_none() || server_addrs.len() == 0 {
        eprintln!("Usage: {} LB_ADDR SERVER_ADDRS\n", env::args().next().unwrap());
        eprintln!("LB_ADDR      : address and port for the load balancer to listen to");
        eprintln!("SERVER_ADDRS : list of server IP addresses and port");
        return Ok(());
    }
    let lb_addr = lb_addr.unwrap();

    let listener = TcpListener::bind(&lb_addr).await?;
    println!("[L] Listening on {}", lb_addr);

    let mut next_server_index = 0;

    loop {
        match listener.accept().await {
            Ok((socket, client_address)) => {
                let server_address = server_addrs[next_server_index].clone();
                next_server_index = (next_server_index + 1) % server_addrs.len();

                tokio::spawn(async move {
                    if let Err(e) = handle_client(socket, client_address, server_address).await {
                        println!("[L] Failed to connect client {} to server {}. {:?}",
                            client_address,
                            server_address,
                            e
                        );
                    }
                });
            }
            Err(e) => {
                println!("[L] Failed to accept client. {:?}", e);
            }
        };
    }
}

async fn handle_client(mut client_socket: TcpStream, client_addr: SocketAddr, server_addr: SocketAddr) -> io::Result<(u64, u64)> {
    let mut server_socket = TcpStream::connect(server_addr).await?;
    println!("[L] Connected client at {} to server at {}.", client_addr, server_addr);

    tokio::io::copy_bidirectional(&mut client_socket, &mut server_socket).await
}
