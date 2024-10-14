// src/main.rs
use tokio::runtime::Runtime;
use std::sync::{Arc, Mutex};
use crate::blockchain::Blockchain;
use crate::network::Network;

#[tokio::main]
async fn main() {
    let blockchain = Arc::new(Mutex::new(Blockchain::new()));
    let mut network = Network {
        peers: vec![],
        blockchain: Arc::clone(&blockchain),
    };
    let addr = "127.0.0.1:6000";
    network.start_server(addr).await;
    cli::run_cli(Arc::clone(&blockchain));
}