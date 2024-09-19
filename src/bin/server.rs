use std::env;
use std::io;
use tcp_load_balancer::run_server;


#[tokio::main]
async fn main() -> io::Result<()> {
    let address = {
        let args: Vec<_> = env::args().collect();

        if args.len() < 2 {
            "127.0.0.1:8080".to_string()
        } else {
            args.into_iter().skip(1).next().unwrap()
        }
    };

    run_server(&address, true).await
}

