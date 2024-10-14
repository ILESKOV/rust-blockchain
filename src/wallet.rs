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

    pub fn save_to_file(&self, filename: &str) -> Result<(), Box<dyn std::error::Error>> {
        let verification_key = VerificationKey::from(&self.signing_key);
        let public_key_hex = hex::encode(verification_key.as_ref());
        let secret_key_hex = hex::encode(self.signing_key.as_ref());
        let data = format!("{}\n{}", public_key_hex, secret_key_hex);
        fs::write(filename, data)?;
        Ok(())
    }
    
    pub fn load_from_file(filename: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let contents = fs::read_to_string(filename)?;
        let lines: Vec<&str> = contents.lines().collect();
        if lines.len() < 2 {
            return Err("Invalid wallet file format".into());
        }
        let secret_key_bytes = hex::decode(lines[1])?;
        let signing_key = SigningKey::try_from(secret_key_bytes.as_slice())?;
        Ok(Wallet::from_signing_key(signing_key))
    }    

    pub fn public_key_hex(&self) -> String {
        let verification_key = VerificationKey::from(&self.signing_key);
        hex::encode(verification_key.as_ref())
    }

    pub fn exists(filename: &str) -> bool {
        Path::new(filename).exists()
    }
}
