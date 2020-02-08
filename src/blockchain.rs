use sha2::{Sha256, Digest};
use byteorder::{LittleEndian, WriteBytesExt};
use chrono::{DateTime, Local, Utc};
use std::fmt;
use crate::keypair;

pub struct Blockchain<'a> {
    blocks: Vec<Block<'a>>,

}

impl<'a> Blockchain<'a> {
    pub fn calculate_balance(&self, address: &'a str) -> i64 {
        let mut balance = 0;
        for block in self.blocks.iter() {
            for signed_transaction in block.transactions.iter() {
                if signed_transaction.transaction.from == address {
                    balance -= signed_transaction.transaction.amount;
                } else if signed_transaction.transaction.to == address {
                    balance += signed_transaction.transaction.amount;
                }
            }
        }
        balance
    }
}

pub struct Block<'a> {
    index: u64,
    transactions: Vec<SignedTransaction<'a>>,
    previous_hash: &'a str,
    current_hash: &'a str,
    timestamp: i64,
    nonce: i64
}

impl<'a> Block<'a> {
    // pub fn create_genesis_block() -> Block<'a> {
    // }
    
    pub fn create(index: u64, transactions: Vec<SignedTransaction<'a>>, previous_hash: &'a str, timestamp: i64, nonce: Option<i64>) -> Block<'a> {
        let i = nonce.unwrap_or(0);
        let block = Block {
            index: index,
            transactions: transactions,
            previous_hash: previous_hash,
            current_hash: "",
            timestamp: timestamp,
            nonce: i
        };
        block.current_hash = "";
        block
    }
}


pub struct Transaction<'a> {
    from: &'a str,
    to: &'a str,
    timestamp: i64,
    amount: i64
}

impl<'a> Transaction<'a> {
    pub fn new(to: &'a str, from: &'a str, timestamp: i64, amount: i64) -> Transaction<'a> {
        Transaction {
            to: to,
            from: from,
            timestamp: timestamp,
            amount: amount
        }
    }

    pub fn create(to: &'a str, from: &'a str, amount: i64) -> Transaction<'a> {
        let local_time = Local::now();
        let utc_time = DateTime::<Utc>::from_utc(local_time.naive_utc(), Utc);
        // let mut timestamp: [u8; mem::size_of::<i64>()] = Default::default();
        // timestamp.as_mut()
            // .write_i64::<LittleEndian>(utc_time.timestamp())
            // .expect("Unable to write");

        Transaction {
            to: to,
            from: from,
            timestamp: utc_time.timestamp(),
            amount: amount
        }
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        let mut amount = [0u8; std::mem::size_of::<i64>()];
        amount.as_mut()
            .write_i64::<LittleEndian>(self.amount)
            .expect("Unable to write");

        let mut timestamp = [0u8; std::mem::size_of::<i64>()];
        amount.as_mut()
            .write_i64::<LittleEndian>(self.timestamp)
            .expect("Unable to write");
        
        [self.to.as_bytes(), self.from.as_bytes(), &amount, &timestamp].concat()
    }

    pub fn as_hash(&self) -> [u8; 32] {
        let mut hasher = Sha256::new();
        hasher.input(self.as_bytes());
        let result = hasher.result();

        let mut hash: [u8; 32] = Default::default();
        hash.copy_from_slice(result.as_slice());
        hash
    }
}

impl<'a> fmt::Display for Transaction<'a> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.write_fmt(format_args!("Transaction(to: {}, from: {}, amount: {})", self.to, self.from, self.amount))
    }
}

pub struct SignedTransaction<'a> {
    transaction: Transaction<'a>,
    signature: String,
    // hash: [u8; 32]
    hash: String
}

impl<'a> SignedTransaction<'a> {
    pub fn new(to: &'a str, from: &'a str, timestamp:i64, amount: i64, signature: &'a str, hash: &'a str) -> SignedTransaction<'a>{
        let transaction = Transaction::new(to, from, timestamp, amount);
        SignedTransaction {
            transaction: transaction,
            signature: signature.to_string(),
            hash: hash.to_string()
        }
    }

    pub fn create(transaction: Transaction<'a>, keypair: &keypair::Keypair) -> SignedTransaction<'a> {
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

    pub fn validate(&self) -> bool {
        let mut hasher = Sha256::new();

        hasher.input(self.transaction.as_bytes());
        hasher.input(self.signature.as_bytes());
        let result = hasher.result();

        let mut hash: [u8; 32] = Default::default();
        hash.copy_from_slice(result.as_slice());

        let is_verified = keypair::verify_signature(self.transaction.from, &self.signature, &self.transaction.as_hash());

        hex::encode(hash) == self.hash && is_verified
    }
}

impl<'a> fmt::Display for SignedTransaction<'a> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.write_fmt(format_args!("SignedTransaction(transaction: {}, signature: {}, hash: {:?})", self.transaction, self.signature, self.hash))
    }
}
