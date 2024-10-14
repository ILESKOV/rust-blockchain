// src/network.rs

use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt};
use serde_json::Value;
use std::sync::{Arc, Mutex};
use crate::blockchain::Blockchain;

pub struct Network {
    pub blockchain: Arc<Mutex<Blockchain>>,
}

impl Network {
    pub fn new(blockchain: Arc<Mutex<Blockchain>>) -> Self {
        Network { blockchain }
    }

    pub async fn start_server(&self, addr: &str) {
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
}

async fn handle_connection(mut socket: TcpStream, blockchain: Arc<Mutex<Blockchain>>) {
    let mut buffer = [0u8; 4096];
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
                // Example handling: if message contains a new block
                if let Some(block) = msg.get("new_block") {
                    // Deserialize the block
                    let _new_block: crate::block::Block = match serde_json::from_value(block.clone()) {
                        Ok(b) => b,
                        Err(e) => {
                            eprintln!("Failed to deserialize block: {}", e);
                            continue;
                        }
                    };

                    // Add the block to the blockchain
                    let bc = blockchain.lock().unwrap();
                    // Implement a method to add a block (e.g., add_block)
                    // bc.add_block(new_block);
                    // For now, we'll assume such a method exists
                    // After adding, save the blockchain
                    if let Err(e) = bc.save_to_file("blockchain.json") {
                        eprintln!("Failed to save blockchain: {}", e);
                    }
                }
                // Implement other message handling as needed
            }
            Err(e) => {
                eprintln!("Failed to parse message: {}", e);
            }
        }
    }
}
