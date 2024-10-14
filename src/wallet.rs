// src/wallet.rs

use ed25519_dalek::{Keypair, PublicKey, SecretKey, Signature, Signer};
use rand::rngs::OsRng;
use serde::{Serialize, Deserialize};
use std::fs;
use std::path::Path;

#[derive(Serialize, Deserialize)]
pub struct Wallet {
    pub public_key: PublicKey,
    #[serde(skip_serializing, skip_deserializing)]
    pub secret_key: SecretKey,
}

impl Wallet {
    pub fn new() -> Self {
        let mut csprng = OsRng {};
        let keypair = Keypair::generate(&mut csprng);
        Wallet {
            public_key: keypair.public,
            secret_key: keypair.secret,
        }
    }

    pub fn from_keys(public_key: PublicKey, secret_key: SecretKey) -> Self {
        Wallet { public_key, secret_key }
    }

    pub fn sign(&self, message: &[u8]) -> Signature {
        let keypair = Keypair {
            secret: self.secret_key.clone(),
            public: self.public_key.clone(),
        };
        keypair.sign(message)
    }

    pub fn save_to_file(&self, filename: &str) {
        let public_key_hex = hex::encode(self.public_key.to_bytes());
        let secret_key_hex = hex::encode(self.secret_key.to_bytes());
        let data = format!("{}\n{}", public_key_hex, secret_key_hex);
        fs::write(filename, data).expect("Unable to save wallet");
    }

    pub fn load_from_file(filename: &str) -> Self {
        let contents = fs::read_to_string(filename).expect("Unable to read wallet file");
        let lines: Vec<&str> = contents.lines().collect();
        let public_key_bytes = hex::decode(lines[0]).unwrap();
        let secret_key_bytes = hex::decode(lines[1]).unwrap();
        let public_key = PublicKey::from_bytes(&public_key_bytes).unwrap();
        let secret_key = SecretKey::from_bytes(&secret_key_bytes).unwrap();
        Wallet::from_keys(public_key, secret_key)
    }

    pub fn exists(filename: &str) -> bool {
        Path::new(filename).exists()
    }
}