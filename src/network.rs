use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use serde_json::Value;
use tokio::sync::{Mutex};
use std::sync::Arc;
use crate::blockchain::Blockchain;
use log::{info, error};
use std::collections::HashSet;
use std::net::SocketAddr;

pub struct Network {
    pub blockchain: Arc<Mutex<Blockchain>>,
    pub peers: Arc<Mutex<HashSet<SocketAddr>>>,
}

impl Network {
    pub fn new(blockchain: Arc<Mutex<Blockchain>>) -> Self {
        Network {
            blockchain,
            peers: Arc::new(Mutex::new(HashSet::new())),
        }
    }

    pub async fn start_server(&self, addr: &str) {
        let listener = TcpListener::bind(addr).await.unwrap();
        info!("Node listening on {}", addr);

        loop {
            let (socket, peer_addr) = listener.accept().await.unwrap();
            let blockchain = Arc::clone(&self.blockchain);
            let peers = Arc::clone(&self.peers);

            // Add the new peer to the peer list
            {
                let mut peers_guard = peers.lock().await;
                peers_guard.insert(peer_addr);
            }

            tokio::spawn(async move {
                handle_connection(socket, blockchain, peers).await;
            });
        }
    }

    pub async fn connect_to_peer(&self, addr: &str) -> Result<(), String> {
        match TcpStream::connect(addr).await {
            Ok(mut stream) => {
                info!("Connected to peer at {}", addr);
                let peers = Arc::clone(&self.peers);
    
                // Add the peer to the peer list
                {
                    let mut peers_guard = peers.lock().await;
                    if let Ok(socket_addr) = addr.parse::<SocketAddr>() {
                        peers_guard.insert(socket_addr);
                    }
                }
    
                // Send initial request for blockchain synchronization
                if let Ok(blockchain) = serde_json::to_string(&self.blockchain.lock().await.chain) {
                    if let Err(e) = stream.write_all(blockchain.as_bytes()).await {
                        error!("Failed to send blockchain data to peer: {}", e);
                        return Err(format!("Failed to send blockchain data to peer: {}", e));
                    }
                }
                Ok(())
            }
            Err(e) => {
                error!("Failed to connect to peer at {}: {}", addr, e);
                Err(format!("Failed to connect to peer at {}: {}", addr, e))
            }
        }
    }

    pub async fn get_peers(&self) -> Vec<SocketAddr> {
        let peers_guard = self.peers.lock().await;
        peers_guard.iter().cloned().collect()
    }
}

async fn handle_connection(mut socket: TcpStream, blockchain: Arc<Mutex<Blockchain>>, peers: Arc<Mutex<HashSet<SocketAddr>>>) {
    let mut buffer = [0u8; 1024];
    loop {
        let n = match socket.read(&mut buffer).await {
            Ok(n) if n == 0 => {
                // Connection closed
                let peer_addr = socket.peer_addr().unwrap();
                {
                    let mut peers_guard = peers.lock().await;
                    peers_guard.remove(&peer_addr);
                }
                info!("Connection closed: {}", peer_addr);
                return;
            }
            Ok(n) => n,
            Err(_) => {
                error!("Failed to read from socket");
                return;
            }
        };

        let received = String::from_utf8_lossy(&buffer[..n]);
        match serde_json::from_str::<Value>(&received) {
            Ok(msg) => {
                // Handle the received message
                info!("Received: {:?}", msg);
                if let Some(chain) = msg.get("chain") {
                    let mut blockchain_guard = blockchain.lock().await;
                    let new_chain: Vec<_> = serde_json::from_value(chain.clone()).unwrap_or_else(|_| blockchain_guard.chain.clone());
                    if new_chain.len() > blockchain_guard.chain.len() {
                        blockchain_guard.chain = new_chain;
                        info!("Blockchain updated from peer");
                    }
                }
            }
            Err(e) => {
                error!("Failed to parse message: {}", e);
            }
        }
    }
}