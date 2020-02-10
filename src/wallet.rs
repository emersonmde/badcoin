use crate::keypair::Keypair;
use crate::blockchain::Transaction;
use crate::blockchain::SignedTransaction;

pub struct Wallet {
    pub keypair: Keypair,
}

impl Wallet {
    // Create a new ECC keypair
    pub fn new() -> Wallet {
        Wallet {
            keypair: Keypair::new()
        }
    }

    // Create new wallet from an existing private key
    pub fn new_from_key(key: String) -> Wallet {
        Wallet {
            keypair: Keypair::new_from_slice(key.as_bytes())
        }
    }

    // Send some coins
    pub fn send(&self, to: &str, amount: i64) {
        let public_key = self.keypair.export_public_key();
        let transaction = Transaction::create(to, &public_key, amount);

        // Sign the new transaction
        let _ = SignedTransaction::create(transaction, &self.keypair);
    }
}