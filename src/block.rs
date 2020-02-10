use crate::signed_transaction::SignedTransaction;
use byteorder::{LittleEndian, WriteBytesExt};
use chrono::{DateTime, Local, Utc};
use sha2::{Sha256, Digest};

pub struct Block {
    pub transactions: Vec<SignedTransaction>,
    pub index: u64,
    pub previous_hash: String,
    pub hash: String,
    pub timestamp: i64,
    pub nonce: u64
}

impl<'a> Block {
    pub const MAX_TRANSACTIONS: i64 = 100;

    // Recreate a block if all fields are known
    pub fn new(index: u64, transactions: Vec<SignedTransaction>, previous_hash: &'a str, timestamp: i64, nonce: u64) -> Block {
        let hash = Block::calculate_hash(index, &transactions, previous_hash, timestamp, nonce);

        Block {
            index: index,
            transactions: transactions,
            previous_hash: previous_hash.to_string(),
            hash: hash,
            timestamp: timestamp,
            nonce: nonce
        }
    }

    // Create a new block
    //
    // Takes a list of transactions and computes the hash for the new block
    pub fn create(index: u64, transactions: Vec<SignedTransaction>, previous_hash: &'a str, nonce: u64) -> Block {
        let local_time = Local::now();
        let utc_time = DateTime::<Utc>::from_utc(local_time.naive_utc(), Utc);

        let hash = Block::calculate_hash(index, &transactions, previous_hash, utc_time.timestamp(), nonce);

        Block {
            index: index,
            transactions: transactions,
            previous_hash: previous_hash.to_string(),
            hash: hash,
            timestamp: utc_time.timestamp(),
            nonce: nonce
        }
    }

    // Calculate the hash for all block fields
    pub fn calculate_hash(index: u64, transactions: &'a Vec<SignedTransaction>, previous_hash: &'a str, timestamp: i64, nonce: u64) -> String {
        let mut hasher = Sha256::new();

        let mut index_bytes = [0u8; std::mem::size_of::<i64>()];
        index_bytes.as_mut()
            .write_u64::<LittleEndian>(index)
            .expect("Unable to serialize index");
        hasher.input(index_bytes);

        for transaction in transactions {
            hasher.input(transaction.as_bytes())
        }

        hasher.input(previous_hash.as_bytes());

        let mut timestamp_bytes = [0u8; std::mem::size_of::<i64>()];
        timestamp_bytes.as_mut()
            .write_i64::<LittleEndian>(timestamp)
            .expect("Unable to serialize timestamp");
        hasher.input(timestamp_bytes);

        let mut nonce_bytes = [0u8; std::mem::size_of::<u64>()];
        nonce_bytes.as_mut()
            .write_u64::<LittleEndian>(nonce)
            .expect("Unable to serialize timestamp");
        hasher.input(nonce_bytes);

        let result = hasher.result();
        hex::encode(result)
    }
    
    // A small helper function to update the nonce and rehash the block
    pub fn update_nonce(&mut self, nonce: u64) {
        self.nonce = nonce;
        self.hash = self.as_hash();
    }

    // Calculae the hash for the current block
    pub fn as_hash(&self) -> String {
        Block::calculate_hash(self.index, &self.transactions, &self.previous_hash, self.timestamp, self.nonce)
    }

    // Validates the hash for a block
    pub fn is_valid(&self) -> bool {
        self.hash == self.as_hash()
    }
}