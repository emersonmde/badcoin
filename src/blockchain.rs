use sha2::{Sha256, Digest};
use byteorder::{LittleEndian, WriteBytesExt};
use chrono::{DateTime, Local, Utc};
use std::fmt;
use crate::keypair;
use std::cell::RefCell;

pub struct Blockchain {
    genesis_hash: String,
    blocks: Vec<Block>,
    unconfirmed_transactions: RefCell<Vec<SignedTransaction>>
}

// TODO: implement forked chain repair
impl<'a> Blockchain {
    pub fn new() -> Blockchain {
        let mut blocks = Vec::new();

        let keypair = keypair::Keypair::new();

        let transaction = Transaction::create(&keypair.export_public_key(), "0", 100);
        let signed_transaction = SignedTransaction::create(transaction, &keypair);

        let mut transactions = Vec::new();
        transactions.push(signed_transaction);
        let block = Block::new(0, transactions, "", 0, 0);
        let hash = block.as_hash();
        println!("Validate block: {}", block.is_valid());
        blocks.push(block);
        Blockchain {
            genesis_hash: hash,
            blocks: blocks,
            unconfirmed_transactions: RefCell::new(Vec::new())
        }
    }

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

    pub fn validate_block(&self, block: &Block) -> Result<bool, String> {
        if !(block.is_valid()) {
            return Err("Invalid block".to_string());
        }

        if block.index < 0 {
            return Err("Invalid block index".to_string());
        }
        if block.index == 0 {
            if block.as_hash() == self.genesis_hash {
                return Ok(true);
            } else {
                return Err("Unable to validate genesis block".to_string());
            }
        }

        let previous_block = self.find_block_by_index(block.index - 1);
        match previous_block {
            Ok(b) => {
                if block.previous_hash != b.hash {
                    return Err("Invalid previous block reference".to_string());
                }
            },
            Err(s) => return Err(s)
        }

        Ok(true)
    }

    pub fn mine_block(&mut self, reward_address: &str) {
        let mut transactions: Vec<SignedTransaction> = Vec::new();
        let latest_block = self.blocks.last().expect("Unknown latest block");
        let new_index = latest_block.index + 1;
        let previous_hash = &latest_block.hash;

        // todo: unconfirmed transactions is empty
        for i in 0..Block::MAX_TRANSACTIONS {
            let transaction_result = self.unconfirmed_transactions.get_mut().pop();
            if transaction_result.is_none() {
                break;
            }

            let transaction = transaction_result.unwrap();

            // TODO: this should already be covered by validate?
            // if hex::encode(result.as_hash()) != result.hash {
            //     continue;
            // }
            if !(transaction.validate()) {
                continue;
            }

            for t in transactions.iter() {
                if transaction.hash == t.hash {
                    continue;
                }
            }

            // TODO: should all transactions for all blocks be checked for uniqueness?

            transactions.push(transaction);
        }

        // TODO: add reward transaction
        // TODO: add transactions to a new block


        // TODO: hash the new block
        // TODO: if block hash doesnt start with '0000' inc nonce, and rehash
        let mut block = Block::create(new_index, transactions, previous_hash, 0);

        // TODO: use do while so first nonce isnt re-calculated
        for i in 0..std::u64::MAX {
            block.update_nonce(i);
            // TODO: prime #s?
            if &block.hash[0..4] == "0000" {
                break;
            }
        }

        println!("PRIFT");
        println!("Block hash {}", block.hash);

        // TODO: Once hash starts with '0000'
        // TODO: profit

        // TODO: setup increasing reward/decreasing difficulty

    }

    fn find_block_by_index(&self, index: u64) -> Result<&Block, String> {
        // TODO: Not even sure if this method is needed, 
        // if so no need to iterate through each block
        for block in self.blocks.iter() {
            if block.index == index {
                return Ok(&block);
            }
        }

        Err("Unable to find block".to_string())
    }

    fn find_block_by_hash(&self, hash: &str) -> Result<&Block, String> {
        // TODO: Not even sure if this method is needed, 
        // if so could be replaced with a merkle tree
        for block in self.blocks.iter() {
            if block.hash == hash {
                return Ok(&block);
            }
        }

        Err("Unable to find block".to_string())
    }
}

pub struct Block {
    index: u64,
    transactions: Vec<SignedTransaction>,
    previous_hash: String,
    hash: String,
    timestamp: i64,
    nonce: u64
}

impl<'a> Block {
    const MAX_TRANSACTIONS: i64 = 100;

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
    
    pub fn update_nonce(&mut self, nonce: u64) {
        self.nonce = nonce;
        self.hash = self.as_hash();
    }

    pub fn as_hash(&self) -> String {
        Block::calculate_hash(self.index, &self.transactions, &self.previous_hash, self.timestamp, self.nonce)
    }

    pub fn is_valid(&self) -> bool {
        self.hash == self.as_hash()
    }
}


pub struct Transaction {
    from: String,
    to: String,
    timestamp: i64,
    amount: i64
}

impl<'a> Transaction {
    pub fn new(to: &'a str, from: &'a str, timestamp: i64, amount: i64) -> Transaction {
        Transaction {
            to: to.to_string(),
            from: from.to_string(),
            timestamp: timestamp,
            amount: amount
        }
    }

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

pub struct SignedTransaction {
    transaction: Transaction,
    signature: String,
    // hash: [u8; 32]
    hash: String
}

impl<'a> SignedTransaction {
    pub fn new(to: &'a str, from: &'a str, timestamp:i64, amount: i64, signature: &'a str, hash: &'a str) -> SignedTransaction {
        let transaction = Transaction::new(to, from, timestamp, amount);
        SignedTransaction {
            transaction: transaction,
            signature: signature.to_string(),
            hash: hash.to_string()
        }
    }

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

    pub fn validate(&self) -> bool {
        let mut hasher = Sha256::new();

        hasher.input(self.transaction.as_bytes());
        hasher.input(self.signature.as_bytes());
        let result = hasher.result();

        let mut hash: [u8; 32] = Default::default();
        hash.copy_from_slice(result.as_slice());

        let is_verified = keypair::verify_signature(&self.transaction.from, &self.signature, &self.transaction.as_hash());

        hex::encode(hash) == self.hash && is_verified
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        let transaction_bytes = self.transaction.as_bytes();
        [transaction_bytes, self.signature.as_bytes().to_vec(), self.hash.as_bytes().to_vec()].concat()
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

impl<'a> fmt::Display for SignedTransaction {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.write_fmt(format_args!("SignedTransaction(transaction: {}, signature: {}, hash: {:?})", self.transaction, self.signature, self.hash))
    }
}
