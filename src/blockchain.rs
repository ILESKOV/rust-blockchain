// src/blockchain.rs

use crate::block::Block;
use crate::transaction::Transaction;
use std::collections::VecDeque;
use crate::zk_proofs::verify_transaction_proof;
use log::{info, error};
use serde::{Serialize, Deserialize};
use std::fs::File;
use std::io::{self, Read, Write};
use serde_json;

#[derive(Serialize, Deserialize, Debug)]
pub struct Blockchain {
    pub chain: Vec<Block>,
    pub pending_transactions: VecDeque<Transaction>,
    pub difficulty: u32,
}

impl Blockchain {
    pub fn new() -> Self {
        let mut blockchain = Blockchain {
            chain: Vec::new(),
            pending_transactions: VecDeque::new(),
            difficulty: 2,
        };
        let genesis_block = blockchain.create_genesis_block();
        blockchain.chain.push(genesis_block);
        blockchain
    }

    fn create_genesis_block(&self) -> Block {
        Block::new(
            0,
            String::from("0"),
            vec![],
        )
    }

    pub fn get_latest_block(&self) -> &Block {
        self.chain.last().unwrap()
    }

    pub fn add_transaction(&mut self, transaction: Transaction) {
        if !transaction.is_valid() {
            eprintln!("Invalid transaction");
            return;
        }
        self.pending_transactions.push_back(transaction);
    }

    pub fn mine_pending_transactions(&mut self, miner_address: &str) {
        let previous_hash = self.get_latest_block().hash.clone();
        let mut transactions = vec![];

        // Collect pending transactions up to a limit (e.g., 10)
        for _ in 0..10 {
            if let Some(tx) = self.pending_transactions.pop_front() {
                transactions.push(tx);
            } else {
                break;
            }
        }

        // Verify zk-SNARK proofs for each transaction
        for tx in &transactions {
            if !verify_transaction_proof(&tx.proof) {
                error!("Invalid zk-SNARK proof in transaction");
                return;
            }
        }

        // Create a reward transaction for the miner
        let reward_tx = Transaction::new_reward(miner_address.to_string());

        // Add the reward transaction to the transactions being added to the new block
        transactions.push(reward_tx.clone());
        info!("Reward transaction created for miner: {}", miner_address);

        // Create the new block with all transactions, including the mining reward
        let mut block = Block::new(
            self.chain.len() as u64,
            previous_hash,
            transactions,
        );

        // Proof of Work
        self.proof_of_work(&mut block);
        self.chain.push(block.clone()); // Clone the block before pushing
        info!("Block mined: {}", block.hash);

        // Debugging: Log the transactions in the mined block
        for (i, tx) in block.transactions.iter().enumerate() {
            info!("Transaction {} in block: sender = {}, recipient = {}, amount = {}", i, tx.sender, tx.recipient, tx.amount);
        }
    }

    pub fn get_balance(&self, address: &str) -> u64 {
        let mut balance: i64 = 0; // Using i64 to handle negative balances temporarily

        // Iterate over each block in the chain
        for block in &self.chain {
            // Iterate over each transaction in the block
            for tx in &block.transactions {
                // If the address is the sender, decrease the balance
                if tx.sender == address {
                    balance -= tx.amount as i64;
                }
                // If the address is the recipient, increase the balance
                if tx.recipient == address {
                    balance += tx.amount as i64;
                }
            }
        }

        // If balance is negative, return 0, otherwise return the actual balance
        if balance < 0 {
            0
        } else {
            balance as u64
        }
    }

    fn proof_of_work(&self, block: &mut Block) {
        // Increment the nonce until a valid hash is found (based on difficulty)
        while &block.hash[..self.difficulty as usize] != &"0".repeat(self.difficulty as usize) {
            block.nonce += 1;
            block.hash = block.calculate_hash();
        }
        println!("Block mined: {}", block.hash);
    }

    /// Saves the current blockchain state to a file.
    pub fn save_to_file(&self, filename: &str) -> io::Result<()> {
        let serialized = serde_json::to_string_pretty(&self)?;
        let mut file = File::create(filename)?;
        file.write_all(serialized.as_bytes())?;
        Ok(())
    }

    /// Loads the blockchain state from a file.
    pub fn load_from_file(filename: &str) -> io::Result<Self> {
        let mut file = File::open(filename)?;
        let mut data = String::new();
        file.read_to_string(&mut data)?;
        let blockchain: Blockchain = serde_json::from_str(&data)?;
        Ok(blockchain)
    }
}
