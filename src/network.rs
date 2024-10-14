// src/network.rs
use tokio::net::{TcpListener, TcpStream};
use tokio::prelude::*;
use serde_json::{Result as SerdeResult, Value};
use std::sync::{Arc, Mutex};
use crate::blockchain::Blockchain;

pub struct Network {
    pub peers: Vec<String>,
    pub blockchain: Arc<Mutex<Blockchain>>,
}

impl Network {
    pub async fn start_server(&mut self, addr: &str) {
        let listener = TcpListener::bind(addr).await.unwrap();
        println!("Node listening on {}", addr);

        loop {
            let (socket, _) = listener.accept().await.unwrap();
            let blockchain = Arc::clone(&self.blockchain);
            tokio::spawn(async move {
                handle_connection(socket, blockchain).await;
            });
        }
    }

    // Additional methods for peer discovery will be implemented later
}

async fn handle_connection(mut socket: TcpStream, blockchain: Arc<Mutex<Blockchain>>) {
    let mut buffer = [0u8; 1024];
    loop {
        let n = match socket.read(&mut buffer).await {
            Ok(n) if n == 0 => return, // Connection closed
            Ok(n) => n,
            Err(_) => {
                eprintln!("Failed to read from socket");
                return;
            }
        };

        let received = String::from_utf8_lossy(&buffer[..n]);
        match serde_json::from_str::<Value>(&received) {
            Ok(msg) => {
                // Handle the received message
                println!("Received: {:?}", msg);
                // Respond or update blockchain accordingly
            }
            Err(e) => {
                eprintln!("Failed to parse message: {}", e);
            }
        }
    }
}