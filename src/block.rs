// src/block.rs
use serde::{Serialize, Deserialize};
use crate::transaction::Transaction;
use sha2::{Sha256, Digest};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Block {
    pub index: u64,
    pub timestamp: u128,
    pub previous_hash: String,
    pub nonce: u64,
    pub transactions: Vec<Transaction>,
    pub hash: String,
}

impl Block {
    pub fn new(index: u64, timestamp: u128, previous_hash: String, transactions: Vec<Transaction>) -> Self {
        let nonce = 0;
        let mut block = Block {
            index,
            timestamp,
            previous_hash,
            nonce,
            transactions,
            hash: String::new(),
        };
        block.hash = block.calculate_hash();
        block
    }

    pub fn calculate_hash(&self) -> String {
        let data = serde_json::to_string(self).unwrap();
        let mut hasher = Sha256::new();
        hasher.update(data);
        let result = hasher.finalize();
        hex::encode(result)
    }
}