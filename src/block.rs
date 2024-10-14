// src/block.rs

use serde::{Serialize, Deserialize};
use crate::transaction::Transaction;
use sha2::{Sha256, Digest};
use chrono::Utc;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Block {
    pub index: u64,
    pub timestamp: i64,
    pub previous_hash: String,
    pub nonce: u64,
    pub transactions: Vec<Transaction>,
    pub hash: String,
}

impl Block {
    pub fn new(index: u64, previous_hash: String, transactions: Vec<Transaction>) -> Self {
        let timestamp = Utc::now().timestamp();
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
        hasher.update(data.as_bytes());
        let result = hasher.finalize();
        hex::encode(result)
    }
}