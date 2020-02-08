use crate::keypair::Keypair;
use crate::blockchain::Transaction;
use crate::blockchain::SignedTransaction;

pub struct Wallet {
    keypair: Keypair,
}

impl Wallet {
    pub fn new() -> Wallet {
        Wallet {
            keypair: Keypair::new()
        }
    }

    pub fn new_from_key(key: String) -> Wallet {
        Wallet {
            keypair: Keypair::new_from_slice(key.as_bytes())
        }
    }

    pub fn send(&self, to: &str, amount: i64) {
        let public_key = self.keypair.export_public_key();
        let transaction = Transaction::create(to, &public_key, amount);

        let a = SignedTransaction::create(transaction, &self.keypair);
        println!("{}", a);

        println!("validate {}", a.validate());
    }
}