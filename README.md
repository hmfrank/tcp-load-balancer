# TCP Load Balancer
Simple TCP load balancer (round-robin).

## How to Use
```shell script
# start some servers
$ cargo run --bin server 127.0.0.1:8081 &
$ cargo run --bin server 127.0.0.1:8082 &
$ cargo run --bin server 127.0.0.1:8083 &
$ cargo run --bin server 127.0.0.1:8084 &

# start the load balancer
$ cargo run --bin load_balancer 127.0.0.1:8080 \
    127.0.0.1:8081 \
    127.0.0.1:8082 \
    127.0.0.1:8083 \
    127.0.0.1:8084
```
To test the load balancer, you have to connect to it with a TCP client.
```shell script
$ cargo run --bin client 127.0.0.1:8080
[C] Connected to 127.0.0.1:8080
[C] Received 26 bytes: "Hello from 127.0.0.1:8081."
```
When the next client connects, we get a response from a different server.
```shell script
# if no argument is given, it defaults to 127.0.0.1:8080
$ cargo run --bin client
[C] Connected to 127.0.0.1:8080
[C] Received 26 bytes: "Hello from 127.0.0.1:8082."
```