// src/blockchain.rs
use crate::block::Block;
use crate::transaction::Transaction;
use std::time::{SystemTime, UNIX_EPOCH};

pub struct Blockchain {
    pub chain: Vec<Block>,
    pub pending_transactions: Vec<Transaction>,
    pub difficulty: u32,
}

impl Blockchain {
    pub fn new() -> Self {
        let mut blockchain = Blockchain {
            chain: Vec::new(),
            pending_transactions: Vec::new(),
            difficulty: 2,
        };
        let genesis_block = blockchain.create_genesis_block();
        blockchain.chain.push(genesis_block);
        blockchain
    }

    fn create_genesis_block(&self) -> Block {
        Block::new(
            0,
            current_timestamp(),
            String::from("0"),
            vec![],
        )
    }

    pub fn get_latest_block(&self) -> &Block {
        self.chain.last().unwrap()
    }

    pub fn add_transaction(&mut self, transaction: Transaction) {
        self.pending_transactions.push(transaction);
    }

    pub fn mine_pending_transactions(&mut self, miner_address: &str) {
        let previous_hash = self.get_latest_block().hash.clone();
        let block = Block::new(
            self.chain.len() as u64,
            current_timestamp(),
            previous_hash,
            self.pending_transactions.clone(),
        );
        // Proof of Work
        let mined_block = self.proof_of_work(block);
        self.chain.push(mined_block);
        self.pending_transactions = vec![];

        // Reward the miner
        let reward_tx = Transaction::new_reward(miner_address.to_string());
        self.pending_transactions.push(reward_tx);
    }

    fn proof_of_work(&self, mut block: Block) -> Block {
        while &block.hash[..self.difficulty as usize] != &"0".repeat(self.difficulty as usize) {
            // Verify transactions using zk-SNARK proofs
            for tx in &block.transactions {
                if !tx.is_valid() {
                    eprintln!("Invalid transaction found during mining");
                    return block; // Or handle accordingly
                }
            }
            block.nonce += 1;
            block.hash = block.calculate_hash();
        }
        block
    }
}

fn current_timestamp() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis()
}

pub fn add_transaction(&mut self, transaction: Transaction) {
    if !transaction.is_valid() {
        eprintln!("Invalid transaction");
        return;
    }
    self.pending_transactions.push(transaction);
}