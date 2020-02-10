use crate::transaction::Transaction;
use crate::keypair;
use sha2::{Sha256, Digest};
use std::fmt;

#[derive(Debug)]
pub struct SignedTransaction {
    pub transaction: Transaction,
    pub signature: String,
    pub hash: String
}

impl<'a> SignedTransaction {
    // Recreates a new SignedTransaction if all fields are known
    pub fn new(to: &'a str, from: &'a str, timestamp:i64, amount: i64, signature: &'a str, hash: &'a str) -> SignedTransaction {
        let transaction = Transaction::new(to, from, timestamp, amount);
        SignedTransaction {
            transaction: transaction,
            signature: signature.to_string(),
            hash: hash.to_string()
        }
    }

    // Signs and hashes a transaction
    pub fn create(transaction: Transaction, keypair: &keypair::Keypair) -> SignedTransaction {
        // TODO: there has to be a better way to convert to slice
        let signature = keypair.sign(&transaction.as_hash()[0..32]);
        let mut hasher = Sha256::new();

        hasher.input(transaction.as_bytes());
        hasher.input(signature.as_bytes());
        let result = hasher.result();

        let mut hash: [u8; 32] = Default::default();
        hash.copy_from_slice(result.as_slice());

        SignedTransaction {
            transaction: transaction,
            signature: signature,
            hash: hex::encode(hash)
        }
    }

    // A helper function to create a reward transaction for miners
    pub fn create_reward(reward_address: &str) -> SignedTransaction {
        let transaction = Transaction::create(reward_address, "0", 10);
        let signature = "0";

        let mut hasher = Sha256::new();
        hasher.input(transaction.as_bytes());
        hasher.input(signature.as_bytes());
        let result = hasher.result();

        let mut hash: [u8; 32] = Default::default();
        hash.copy_from_slice(result.as_slice());

        SignedTransaction {
            transaction: transaction,
            signature: signature.to_string(),
            hash: hex::encode(hash)
        }
    }

    // Verifies the signature and hash for a transaction
    pub fn is_valid(&self) -> bool {
        let mut hasher = Sha256::new();

        hasher.input(self.transaction.as_bytes());
        hasher.input(self.signature.as_bytes());
        let result = hasher.result();

        let mut hash: [u8; 32] = Default::default();
        hash.copy_from_slice(result.as_slice());

        let is_verified = keypair::verify_signature(&self.transaction.from, &self.signature, &self.transaction.as_hash());

        hex::encode(hash) == self.hash && is_verified
    }

    // Converts all transaction fields to a byte vector
    pub fn as_bytes(&self) -> Vec<u8> {
        let transaction_bytes = self.transaction.as_bytes();
        [transaction_bytes, self.signature.as_bytes().to_vec(), self.hash.as_bytes().to_vec()].concat()
    }

    // Hashes a SignedTransaction
    pub fn as_hash(&self) -> [u8; 32] {
        let mut hasher = Sha256::new();
        hasher.input(self.as_bytes());
        let result = hasher.result();

        let mut hash: [u8; 32] = Default::default();
        hash.copy_from_slice(result.as_slice());
        hash
    }
}

impl<'a> fmt::Display for SignedTransaction {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.write_fmt(format_args!("SignedTransaction(transaction: {}, signature: {}, hash: {:?})", self.transaction, self.signature, self.hash))
    }
}