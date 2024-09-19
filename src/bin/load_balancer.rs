use core::net::SocketAddr;
use std::{env, io};
use tcp_load_balancer::run_load_balancer;

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

    run_load_balancer(lb_addr, &server_addrs, true).await
}