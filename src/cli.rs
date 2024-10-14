// src/cli.rs

use clap::{App, Arg, SubCommand};
use crate::wallet::Wallet;
use crate::transaction::Transaction;
use crate::blockchain::Blockchain;
use std::sync::{Arc, Mutex};

pub async fn run_cli(blockchain: Arc<Mutex<Blockchain>>) {
    let matches = App::new("Privacy Blockchain")
        .version("1.0")
        .author("Your Name")
        .about("A Rust-based privacy-preserving blockchain")
        .subcommand(
            SubCommand::with_name("wallet")
                .about("Manage your wallet")
                .subcommand(SubCommand::with_name("create").about("Create a new wallet"))
                .subcommand(SubCommand::with_name("balance").about("Check wallet balance")),
        )
        .subcommand(
            SubCommand::with_name("transaction")
                .about("Create a new transaction")
                .arg(Arg::with_name("recipient").required(true))
                .arg(Arg::with_name("amount").required(true)),
        )
        .subcommand(SubCommand::with_name("mine").about("Mine pending transactions"))
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("wallet") {
        if matches.subcommand_matches("create").is_some() {
            let wallet = Wallet::new();
            wallet.save_to_file("wallet.dat");
            println!("Wallet created and saved to wallet.dat");
            println!("Public Key: {}", wallet.public_key_hex());
        } else if matches.subcommand_matches("balance").is_some() {
            // Implement balance checking
            println!("Balance feature not implemented yet.");
        }
    } else if let Some(matches) = matches.subcommand_matches("transaction") {
        let recipient = matches.value_of("recipient").unwrap();
        let amount = matches.value_of("amount").unwrap().parse::<u64>().unwrap();
        // Load sender's wallet
        if !Wallet::exists("wallet.dat") {
            println!("Wallet not found. Please create one first.");
            return;
        }
        let wallet = Wallet::load_from_file("wallet.dat");
        let mut tx = Transaction::new(
            wallet.public_key_hex(),
            recipient.to_string(),
            amount,
        );
        tx.sign_transaction(&wallet.keypair);
        let mut bc = blockchain.lock().unwrap();
        bc.add_transaction(tx);
        println!("Transaction added to pending transactions.");
    } else if matches.subcommand_matches("mine").is_some() {
        let mut bc = blockchain.lock().unwrap();
        bc.mine_pending_transactions("miner_address");
        println!("Mining complete.");
    } else {
        println!("No valid subcommand was used. Use --help for more information.");
    }
}