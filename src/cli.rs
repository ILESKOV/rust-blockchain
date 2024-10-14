// src/cli.rs
use clap::{App, Arg, SubCommand};
use crate::wallet::Wallet;
use crate::transaction::Transaction;
use crate::blockchain::Blockchain;
use std::sync::{Arc, Mutex};

pub fn run_cli(blockchain: Arc<Mutex<Blockchain>>) {
    let matches = App::new("Privacy Blockchain")
        .version("1.0")
        .author("Your Name")
        .about("A Rust-based privacy-preserving blockchain")
        .subcommand(
            SubCommand::with_name("wallet")
                .about("Manage your wallet")
                .subcommand(SubCommand::with_name("create").about("Create a new wallet")),
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
            println!("Public Key: {}", hex::encode(wallet.public_key.to_bytes()));
            // Save wallet details securely
        }
    } else if let Some(matches) = matches.subcommand_matches("transaction") {
        let recipient = matches.value_of("recipient").unwrap();
        let amount = matches.value_of("amount").unwrap().parse::<u64>().unwrap();
        // Load sender's wallet
        let wallet = Wallet::new(); // Replace with actual loading logic
        let mut tx = Transaction::new(
            hex::encode(wallet.public_key.to_bytes()),
            recipient.to_string(),
            amount,
        );
        tx.sign_transaction(&wallet.secret_key);
        let mut bc = blockchain.lock().unwrap();
        bc.add_transaction(tx);
        println!("Transaction added to pending transactions.");
    } else if matches.subcommand_matches("mine").is_some() {
        let mut bc = blockchain.lock().unwrap();
        bc.mine_pending_transactions("miner_address");
        println!("Mining complete.");
    }
}