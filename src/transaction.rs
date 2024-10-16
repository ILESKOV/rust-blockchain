// src/transaction.rs

use serde::{Serialize, Deserialize};
use ed25519_zebra::{VerificationKey, SigningKey, Signature};
use sha2::{Sha256, Digest};
use crate::zk_proofs::{generate_transaction_proof, ProofData};
use std::convert::{TryFrom, TryInto};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Transaction {
    pub sender: String,
    pub recipient: String,
    pub amount: u64,
    pub signature: Option<String>,
    pub proof: ProofData,
}

impl Transaction {
    pub fn new(sender: String, recipient: String, amount: u64) -> Self {
        let proof = generate_transaction_proof(amount);
        Transaction { sender, recipient, amount, signature: None, proof }
    }

    const MINING_REWARD: u64 = 50;

    pub fn new_reward(recipient: String) -> Self {
        let proof = generate_transaction_proof(Self::MINING_REWARD); // Use constant
        Transaction {
            sender: String::from("System"),
            recipient,
            amount: Self::MINING_REWARD,
            signature: None,
            proof,
        }
    }

    pub fn sign_transaction(&mut self, signing_key: &SigningKey) {
        let message = self.calculate_hash();
        let signature = signing_key.sign(message.as_bytes());
        // Convert the signature into a byte array
        let signature_bytes: [u8; 64] = signature.into();
        self.signature = Some(hex::encode(signature_bytes));
    }

    pub fn is_valid(&self) -> bool {
        if self.sender == "System" {
            return true; // Reward transaction
        }

        if let Some(sig_hex) = &self.signature {
            let signature_bytes = match hex::decode(sig_hex) {
                Ok(bytes) => bytes,
                Err(e) => {
                    eprintln!("Error decoding signature: {}", e);
                    return false;
                }
            };
            let signature_array: [u8; 64] = match signature_bytes.as_slice().try_into() {
                Ok(arr) => arr,
                Err(_) => {
                    eprintln!("Invalid signature length");
                    return false;
                }
            };
            let signature = Signature::from(signature_array);

            let public_key_bytes = match hex::decode(&self.sender) {
                Ok(bytes) => bytes,
                Err(e) => {
                    eprintln!("Error decoding public key: {}", e);
                    return false;
                }
            };
            let verification_key = match VerificationKey::try_from(public_key_bytes.as_slice()) {
                Ok(vk) => vk,
                Err(e) => {
                    eprintln!("Error creating verification key: {}", e);
                    return false;
                }
            };
            let message = self.calculate_hash();
            verification_key.verify(&signature, message.as_bytes()).is_ok()
        } else {
            false
        }
    }

    fn calculate_hash(&self) -> String {
        let data = format!("{}{}{}", self.sender, self.recipient, self.amount);
        let mut hasher = Sha256::new();
        hasher.update(data.as_bytes());
        let result = hasher.finalize();
        hex::encode(result)
    }
}
