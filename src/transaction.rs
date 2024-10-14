// src/transaction.rs
use serde::{Serialize, Deserialize};
use ed25519_dalek::Signature;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Transaction {
    pub sender: String,
    pub recipient: String,
    pub amount: u64,
    pub signature: Option<String>,
    pub proof: Option<String>,
}

impl Transaction {
    pub fn new(sender: String, recipient: String, amount: u64) -> Self {
        let proof = generate_proof(amount);
        Transaction { sender, recipient, amount, signature: None, proof: Some(proof) }
    }

    pub fn sign_transaction(&mut self, private_key: &ed25519_dalek::SecretKey) {
        let message = self.calculate_hash();
        let keypair = ed25519_dalek::Keypair {
            secret: private_key.clone(),
            public: ed25519_dalek::PublicKey::from(private_key),
        };
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
            let public_key = ed25519_dalek::PublicKey::from_bytes(&public_key_bytes).unwrap();
            let message = self.calculate_hash();
            public_key.verify(message.as_bytes(), &signature).is_ok()
        } else {
            false
        }
    }

    fn calculate_hash(&self) -> String {
        let data = format!("{}{}{}", self.sender, self.recipient, self.amount);
        let mut hasher = sha2::Sha256::new();
        hasher.update(data.as_bytes());
        let result = hasher.finalize();
        hex::encode(result)
    } 
}