use clap::{App, Arg, SubCommand};
use crate::wallet::Wallet;
use crate::transaction::Transaction;
use crate::blockchain::Blockchain;
use crate::network::Network;
use std::sync::Arc;
use tokio::sync::Mutex;

pub async fn run_cli(blockchain: Arc<Mutex<Blockchain>>, network: Arc<Mutex<Network>>) {
    let matches = App::new("Privacy Blockchain")
        .version("1.0")
        .author("Your Name")
        .about("A Rust-based privacy-preserving blockchain")
        .arg(
            Arg::with_name("port")
                .long("port")
                .short('p')
                .takes_value(true)
                .default_value("6000")
                .help("Port number for the node"),
        )
        .subcommand(
            SubCommand::with_name("wallet")
                .about("Manage your wallet")
                .subcommand(SubCommand::with_name("create").about("Create a new wallet"))
                .subcommand(SubCommand::with_name("balance").about("Check wallet balance")),
        )
        .subcommand(
            SubCommand::with_name("transaction")
                .about("Create a new transaction")
                .arg(Arg::with_name("recipient").required(true).help("Recipient's public key"))
                .arg(Arg::with_name("amount").required(true).help("Amount to send")),
        )
        .subcommand(SubCommand::with_name("mine").about("Mine pending transactions"))
        .subcommand(
            SubCommand::with_name("connect")
                .about("Connect to a peer node")
                .arg(Arg::with_name("address").required(true).help("Peer address to connect to (e.g., 127.0.0.1:6000)")),
        )
        .subcommand(SubCommand::with_name("peers").about("List connected peers"))
        .subcommand(SubCommand::with_name("status").about("Show blockchain status and peer information"))
        .get_matches();

    // Clone the port value before using it inside the async block
    let port = matches.value_of("port").unwrap().to_string();

    // Start the network server on the specified port
    let network_clone = Arc::clone(&network);
    tokio::spawn(async move {
        network_clone.lock().await.start_server(&format!("127.0.0.1:{}", port)).await;
    });

    // Interactive CLI loop
    loop {
        println!("Enter a command (type 'exit' to quit):");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).expect("Failed to read line");
        let input = input.trim();

        if input == "exit" {
            break;
        }

        let args: Vec<&str> = input.split_whitespace().collect();
        if args.is_empty() {
            continue;
        }

        match args[0] {
            "wallet" => {
                if args.len() > 1 {
                    match args[1] {
                        "create" => {
                            let wallet = Wallet::new();
                            if let Err(e) = wallet.save_to_file("wallet.dat") {
                                eprintln!("Failed to save wallet: {}", e);
                            } else {
                                println!("Wallet created and saved to wallet.dat");
                                println!("Public Key: {}", wallet.public_key_hex());
                            }
                        }
                        "balance" => {
                            if Wallet::exists("wallet.dat") {
                                let wallet = Wallet::load_from_file("wallet.dat").expect("Failed to load wallet");
                                let blockchain = blockchain.lock().await;
                                let balance = blockchain.get_balance(&wallet.public_key_hex());
                                println!("Wallet balance: {}", balance);
                            } else {
                                println!("Wallet not found. Please create one first.");
                            }
                        }
                        _ => println!("Unknown wallet command. Use 'create' or 'balance'."),
                    }
                } else {
                    println!("Usage: wallet <create|balance>");
                }
            }
            "transaction" => {
                if args.len() == 3 {
                    let recipient = args[1];
                    let amount: u64 = match args[2].parse() {
                        Ok(a) => a,
                        Err(_) => {
                            eprintln!("Invalid amount. Please enter a valid number.");
                            continue;
                        }
                    };
                    if !Wallet::exists("wallet.dat") {
                        println!("Wallet not found. Please create one first.");
                        continue;
                    }
                    let wallet = Wallet::load_from_file("wallet.dat").expect("Failed to load wallet");
                    let mut tx = Transaction::new(wallet.public_key_hex(), recipient.to_string(), amount);
                    tx.sign_transaction(&wallet.signing_key);
                    let mut bc = blockchain.lock().await;
                    bc.add_transaction(tx);
                    println!("Transaction added to pending transactions.");

                    if let Err(e) = bc.save_to_file("blockchain.json") {
                        eprintln!("Failed to save blockchain: {}", e);
                    }
                } else {
                    println!("Usage: transaction <recipient> <amount>");
                }
            }
            "mine" => {
                if Wallet::exists("wallet.dat") {
                    let wallet = Wallet::load_from_file("wallet.dat").expect("Unable to load wallet");
                    let mut bc = blockchain.lock().await;
                    bc.mine_pending_transactions(&wallet.public_key_hex());
                    println!("Mining complete. Wallet address: {}", wallet.public_key_hex());

                    if let Err(e) = bc.save_to_file("blockchain.json") {
                        eprintln!("Failed to save blockchain: {}", e);
                    }
                } else {
                    println!("Wallet not found. Please create one first.");
                }
            }
            "connect" => {
                if args.len() == 2 {
                    let address = args[1];
                    if let Err(e) = network.lock().await.connect_to_peer(address).await {
                        eprintln!("Failed to connect to peer: {}", e);
                    } else {
                        println!("Connected to peer: {}", address);
                    }
                } else {
                    println!("Usage: connect <address>");
                }
            }
            "peers" => {
                let peers = network.lock().await.get_peers().await;
                if peers.is_empty() {
                    println!("No peers connected.");
                } else {
                    println!("Connected peers:");
                    for peer in peers {
                        println!("- {}", peer);
                    }
                }
            }
            "status" => {
                let bc = blockchain.lock().await;
                println!("Blockchain status:");
                println!("  Blocks: {}", bc.chain.len());
                println!("  Pending transactions: {}", bc.pending_transactions.len());

                let peers = network.lock().await.get_peers().await;
                println!("Connected peers: {}", peers.len());
                for peer in peers {
                    println!("- {}", peer);
                }
            }
            _ => {
                println!("Unknown command. Use 'wallet', 'transaction', 'mine', 'connect', 'peers', or 'status'.");
            }
        }
    }
}