// src/blockchain.rs

use crate::block::Block;
use crate::transaction::Transaction;
use std::collections::VecDeque;
use crate::zk_proofs::verify_transaction_proof;

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

        // Collect transactions up to a limit (e.g., 10)
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
                eprintln!("Invalid zk-SNARK proof in transaction");
                return;
            }
        }

        let mut block = Block::new(
            self.chain.len() as u64,
            previous_hash,
            transactions,
        );

        // Proof of Work
        self.proof_of_work(&mut block);
        self.chain.push(block);

        // Reward the miner
        let reward_tx = Transaction::new_reward(miner_address.to_string());
        self.pending_transactions.push_back(reward_tx);
    }

    fn proof_of_work(&self, block: &mut Block) {
        while &block.hash[..self.difficulty as usize] != &"0".repeat(self.difficulty as usize) {
            block.nonce += 1;
            block.hash = block.calculate_hash();
        }
        println!("Block mined: {}", block.hash);
    }
}