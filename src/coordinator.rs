use std::env;
use std::net::SocketAddr;
use tokio::{net::UdpSocket, time::{sleep, Duration}};

#[tokio::main]
async fn main() {
    // Collect IPs and ports from command-line arguments
    let args: Vec<String> = env::args().collect();
    
    // Check if there are enough arguments
    if args.len() < 3 || args.len() % 2 != 1 {
        eprintln!("Usage: cargo run --release --bin coordinator -- <ip1> <port1> <ip2> <port2> ...");
        return;
    }

    // Parse the IPs and ports into a vector of node addresses
    let mut nodes = Vec::new();
    for i in (1..args.len()).step_by(2) {
        let ip = &args[i];
        let port: u16 = args[i + 1].parse().expect("Invalid port number");
        let addr = format!("{}:{}", ip, port).parse::<SocketAddr>().unwrap();
        nodes.push(addr);
    }

    // Bind the UDP socket (not used for listening here, just sending)
    let socket = UdpSocket::bind("0.0.0.0:0").await.expect("Failed to bind UDP socket");

    let mut index = 0;

    loop {
        let addr = nodes[index];

        // Send "down" message
        println!("Notifying Node {} to go down", addr);
        let _ = socket.send_to(b"down", addr).await;

        // Wait to simulate downtime
        sleep(Duration::from_secs(5)).await;

        // Send "up" message
        println!("Notifying Node {} to go up", addr);
        let _ = socket.send_to(b"up", addr).await;

        // Short delay before moving to the next node
        sleep(Duration::from_secs(1)).await;

        // Move to the next node in the round-robin sequence
        index = (index + 1) % nodes.len();
    }
}
