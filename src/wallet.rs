// src/wallet.rs

use ed25519_dalek::{Keypair, Signature, Signer};
use rand::rngs::OsRng;
use std::fs;
use std::path::Path;

pub struct Wallet {
    pub keypair: Keypair,
}

impl Wallet {
    pub fn new() -> Self {
        let mut csprng = OsRng{};
        let keypair = Keypair::generate(&mut csprng);
        Wallet { keypair }
    }

    pub fn from_keypair(keypair: Keypair) -> Self {
        Wallet { keypair }
    }

    pub fn sign(&self, message: &[u8]) -> Signature {
        self.keypair.sign(message)
    }

    pub fn save_to_file(&self, filename: &str) {
        let public_key_hex = hex::encode(self.keypair.public.to_bytes());
        let secret_key_hex = hex::encode(self.keypair.secret.to_bytes());
        let data = format!("{}\n{}", public_key_hex, secret_key_hex);
        fs::write(filename, data).expect("Unable to save wallet");
    }

    pub fn load_from_file(filename: &str) -> Self {
        let contents = fs::read_to_string(filename).expect("Unable to read wallet file");
        let lines: Vec<&str> = contents.lines().collect();
        let public_key_bytes = hex::decode(lines[0]).unwrap();
        let secret_key_bytes = hex::decode(lines[1]).unwrap();
        let public_key = ed25519_dalek::PublicKey::from_bytes(&public_key_bytes).unwrap();
        let secret_key = ed25519_dalek::SecretKey::from_bytes(&secret_key_bytes).unwrap();
        let keypair = Keypair { secret: secret_key, public: public_key };
        Wallet::from_keypair(keypair)
    }

    pub fn public_key_hex(&self) -> String {
        hex::encode(self.keypair.public.to_bytes())
    }

    pub fn exists(filename: &str) -> bool {
        Path::new(filename).exists()
    }
}