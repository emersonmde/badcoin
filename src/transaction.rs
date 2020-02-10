use chrono::{DateTime, Local, Utc};
use byteorder::{LittleEndian, WriteBytesExt};
use sha2::{Sha256, Digest};
use std::fmt;

#[derive(Debug)]
pub struct Transaction {
    pub from: String,
    pub to: String,
    pub timestamp: i64,
    pub amount: i64
}

impl<'a> Transaction {
    // Creates a new transaction
    pub fn new(to: &'a str, from: &'a str, timestamp: i64, amount: i64) -> Transaction {
        Transaction {
            to: to.to_string(),
            from: from.to_string(),
            timestamp: timestamp,
            amount: amount
        }
    }

    // Creates a new transaction with the current timestamp
    pub fn create(to: &'a str, from: &'a str, amount: i64) -> Transaction {
        let local_time = Local::now();
        let utc_time = DateTime::<Utc>::from_utc(local_time.naive_utc(), Utc);

        Transaction {
            to: to.to_string(),
            from: from.to_string(),
            timestamp: utc_time.timestamp(),
            amount: amount
        }
    }

    // Converts all transaction fields to a byte vector
    pub fn as_bytes(&self) -> Vec<u8> {
        let mut amount = [0u8; std::mem::size_of::<i64>()];
        amount.as_mut()
            .write_i64::<LittleEndian>(self.amount)
            .expect("Unable to write");

        let timestamp = [0u8; std::mem::size_of::<i64>()];
        amount.as_mut()
            .write_i64::<LittleEndian>(self.timestamp)
            .expect("Unable to write");
        
        [self.to.as_bytes(), self.from.as_bytes(), &amount, &timestamp].concat()
    }

    // Hashes all transaction fields
    pub fn as_hash(&self) -> [u8; 32] {
        let mut hasher = Sha256::new();
        hasher.input(self.as_bytes());
        let result = hasher.result();

        let mut hash: [u8; 32] = Default::default();
        hash.copy_from_slice(result.as_slice());
        hash
    }
}

impl<'a> fmt::Display for Transaction {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.write_fmt(format_args!("Transaction(to: {}, from: {}, amount: {})", self.to, self.from, self.amount))
    }
}