use std::sync::Arc;
use tokio::sync::Mutex;
use privacy_blockchain::blockchain::Blockchain;
use privacy_blockchain::network::Network;
use privacy_blockchain::cli;
use std::path::Path;

#[tokio::main]
async fn main() {
    env_logger::init();

    // Load or create blockchain
    let blockchain = if Path::new("blockchain.json").exists() {
        match Blockchain::load_from_file("blockchain.json") {
            Ok(bc) => bc,
            Err(e) => {
                eprintln!("Failed to load blockchain: {}", e);
                Blockchain::new()
            }
        }
    } else {
        Blockchain::new()
    };

    let blockchain = Arc::new(Mutex::new(blockchain));
    let network = Arc::new(Mutex::new(Network::new(Arc::clone(&blockchain))));

    // Start the networking in a separate task
    let network_clone = Arc::clone(&network);
    tokio::spawn(async move {
        network_clone.lock().await.start_server("127.0.0.1:6000").await;
    });

    // Run the CLI, passing both blockchain and network
    cli::run_cli(Arc::clone(&blockchain), Arc::clone(&network)).await;

    // Save the blockchain state before exiting
    let bc = blockchain.lock().await;
    if let Err(e) = bc.save_to_file("blockchain.json") {
        eprintln!("Failed to save blockchain: {}", e);
    }
}