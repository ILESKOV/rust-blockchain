// src/wallet.rs
use ed25519_dalek::{Keypair, PublicKey, SecretKey, Signature, Signer, Verifier};
use rand::rngs::OsRng;
use serde::{Serialize, Deserialize};

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

    pub fn sign(&self, message: &[u8]) -> Signature {
        let keypair = Keypair {
            secret: self.secret_key.clone(),
            public: self.public_key.clone(),
        };
        keypair.sign(message)
    }

    pub fn verify(&self, message: &[u8], signature: &Signature) -> bool {
        self.public_key.verify(message, signature).is_ok()
    }
}