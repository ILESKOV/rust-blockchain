// src/transaction.rs

use serde::{Serialize, Deserialize};
use ed25519_dalek::{PublicKey, Signature, Verifier, Signer, Keypair};
use sha2::{Sha256, Digest};
use crate::zk_proofs::{generate_transaction_proof, Proof};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Transaction {
    pub sender: String,
    pub recipient: String,
    pub amount: u64,
    pub signature: Option<String>,
    pub proof: Proof,
}

impl Transaction {
    pub fn new(sender: String, recipient: String, amount: u64) -> Self {
        let proof = generate_transaction_proof(amount);
        Transaction { sender, recipient, amount, signature: None, proof }
    }

    pub fn new_reward(recipient: String) -> Self {
        let proof = generate_transaction_proof(50); // Reward amount
        Transaction {
            sender: String::from("System"),
            recipient,
            amount: 50,
            signature: None,
            proof,
        }
    }

    pub fn sign_transaction(&mut self, keypair: &Keypair) {
        if self.sender != hex::encode(keypair.public.to_bytes()) {
            panic!("You cannot sign transactions for other wallets!");
        }
        let message = self.calculate_hash();
        let signature = keypair.sign(message.as_bytes());
        self.signature = Some(hex::encode(signature.to_bytes()));
    }

    pub fn is_valid(&self) -> bool {
        if self.sender == "System" {
            return true; // Reward transaction
        }

        if let Some(sig_hex) = &self.signature {
            let signature_bytes = hex::decode(sig_hex).unwrap();
            let signature = Signature::from_bytes(&signature_bytes).unwrap();
            let public_key_bytes = hex::decode(&self.sender).unwrap();
            let public_key = PublicKey::from_bytes(&public_key_bytes).unwrap();
            let message = self.calculate_hash();
            public_key.verify(message.as_bytes(), &signature).is_ok()
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