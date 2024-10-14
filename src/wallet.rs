// src/wallet.rs

use ed25519_zebra::{SigningKey, VerificationKey, Signature};
use rand::rngs::OsRng;
use std::fs;
use std::path::Path;
use std::convert::TryFrom;

pub struct Wallet {
    pub signing_key: SigningKey,
}

impl Wallet {
    pub fn new() -> Self {
        let mut rng = OsRng;
        let signing_key = SigningKey::new(&mut rng);
        Wallet { signing_key }
    }

    pub fn from_signing_key(signing_key: SigningKey) -> Self {
        Wallet { signing_key }
    }

    pub fn sign(&self, message: &[u8]) -> Signature {
        self.signing_key.sign(message)
    }

    pub fn save_to_file(&self, filename: &str) {
        let verification_key = VerificationKey::from(&self.signing_key);
        let public_key_hex = hex::encode(verification_key.as_ref());
        let secret_key_hex = hex::encode(self.signing_key.as_ref());
        let data = format!("{}\n{}", public_key_hex, secret_key_hex);
        fs::write(filename, data).expect("Unable to save wallet");
    }

    pub fn load_from_file(filename: &str) -> Self {
        let contents = fs::read_to_string(filename).expect("Unable to read wallet file");
        let lines: Vec<&str> = contents.lines().collect();
        let secret_key_bytes = hex::decode(lines[1]).unwrap();
        let signing_key = SigningKey::try_from(secret_key_bytes.as_slice()).unwrap();
        Wallet::from_signing_key(signing_key)
    }

    pub fn public_key_hex(&self) -> String {
        let verification_key = VerificationKey::from(&self.signing_key);
        hex::encode(verification_key.as_ref())
    }

    pub fn exists(filename: &str) -> bool {
        Path::new(filename).exists()
    }
}