use tokio::{net::UdpSocket, time::sleep};
use std::{env, io::Write};

#[tokio::main]
async fn main() {
    // Get IP and port from command-line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: cargo run --release --bin node -- <ip> <port>");
        return;
    }
    let ip = &args[1];
    let port = args[2].parse::<u16>().expect("Invalid port number");

    let address = format!("{}:{}", ip, port);
    let socket = UdpSocket::bind(&address).await.expect("Failed to bind UDP socket");
    println!("Node listening on {}", address);

    let mut buffer = [0; 4];
    let mut down_count = 0;

    loop {
        let (len, _src) = socket.recv_from(&mut buffer).await.expect("Failed to receive data");
        let message = std::str::from_utf8(&buffer[..len]).expect("Invalid UTF-8 message");

        match message {
            "down" => {
                down_count += 1;
                println!("Node {} going down (count: {})", address, down_count);
                std::io::stdout().flush().unwrap();
                // Simulate downtime
                sleep(tokio::time::Duration::from_secs(5)).await;
            }
            "up" => {
                println!("Node {} back up", address);
                std::io::stdout().flush().unwrap();
            }
            _ => {
                println!("Unknown message received");
                std::io::stdout().flush().unwrap();
            }
        }
    }
}
