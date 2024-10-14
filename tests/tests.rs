// tests/tests.rs
use privacy_blockchain::blockchain::Blockchain;
use privacy_blockchain::transaction::Transaction;
use privacy_blockchain::wallet::Wallet;

#[test]
fn test_transaction_creation() {
    let wallet = Wallet::new();
    let mut tx = Transaction::new(
        hex::encode(wallet.public_key.to_bytes()),
        "recipient_address".to_string(),
        100,
    );
    tx.sign_transaction(&wallet.secret_key);
    assert!(tx.is_valid());
}

#[test]
fn test_blockchain() {
    let mut blockchain = Blockchain::new();
    let wallet = Wallet::new();
    let mut tx = Transaction::new(
        hex::encode(wallet.public_key.to_bytes()),
        "recipient_address".to_string(),
        100,
    );
    tx.sign_transaction(&wallet.secret_key);
    blockchain.add_transaction(tx);
    blockchain.mine_pending_transactions("miner_address");
    assert_eq!(blockchain.chain.len(), 2);
}