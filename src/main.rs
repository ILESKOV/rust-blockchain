// src/main.rs

use std::sync::{Arc, Mutex};
use privacy_blockchain::blockchain::Blockchain;
use privacy_blockchain::network::Network;
use privacy_blockchain::cli;

#[tokio::main]
async fn main() {
    let blockchain = Arc::new(Mutex::new(Blockchain::new()));
    let network = Network::new(Arc::clone(&blockchain));

    // Start the networking in a separate task
    tokio::spawn(async move {
        network.start_server("127.0.0.1:6000").await;
    });

    // Run the CLI
    cli::run_cli(Arc::clone(&blockchain)).await;
}