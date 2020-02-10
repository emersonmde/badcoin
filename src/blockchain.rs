use std::cell::RefCell;

pub use crate::keypair::Keypair;
pub use crate::transaction::Transaction;
pub use crate::signed_transaction::SignedTransaction;
pub use crate::block::Block;

pub struct Blockchain {
    genesis_hash: String,
    blocks: Vec<Block>,
    pending_transactions: RefCell<Vec<SignedTransaction>>
}

// TODO: implement forked chain repair
impl<'a> Blockchain {
    // Creates a new blockchain with a genesis block
    // 
    // Current implementation uses an existing keypair for some initial coins to test with
    pub fn new(keypair: &Keypair) -> Blockchain {
        let mut blocks = Vec::new();

        let transaction = Transaction::create(&keypair.export_public_key(), "0", 100);
        let signed_transaction = SignedTransaction::create(transaction, &keypair);

        let mut transactions = Vec::new();
        transactions.push(signed_transaction);
        let block = Block::new(0, transactions, "", 0, 0);
        let hash = block.as_hash();

        blocks.push(block);
        Blockchain {
            genesis_hash: hash,
            blocks: blocks,
            pending_transactions: RefCell::new(Vec::new())
        }
    }

    // Runs through all transactions in the ledger to calculate total balance
    // for an address
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

    // Validates a block or errors
    //
    // Checks if the current block is the genesis block, otherwise verifies the blocks
    // hash. Also checks chain continuity by ensuring the previous hash maps maps to
    // the block with the next smallest ID.
    //
    // TODO: Should the block signature be validated as well?
    pub fn validate_block(&self, block: &Block) -> Result<(), &str> {
        if !(block.is_valid()) {
            return Err("Invalid block");
        }

        if block.index == 0 {
            if block.as_hash() == self.genesis_hash {
                return Ok(());
            } else {
                return Err("Unable to validate genesis block");
            }
        }

        let previous_block = self.find_block_by_index(block.index - 1);
        match previous_block {
            Ok(b) => {
                if block.previous_hash != b.hash {
                    return Err("Invalid previous block reference");
                }
            },
            Err(s) => return Err(s)
        }

        Ok(())
    }

    // Adds a transaction to the pending transactions pool
    pub fn add_pending_transaction(&mut self, transaction: SignedTransaction) {
        self.pending_transactions.get_mut().push(transaction);
    }

    // Mines a block
    //
    // The current mining process:
    //   1. Make sure theres at least 1 to max transactions per block pending
    //   2. Validate each transaction
    //     a) Verify transaction signature
    //     b) Verify transaction hash
    //     c) from address contains enough coins
    //     d) transaction isn't duplicated in pending_transactions
    //     e) transaction doesn't appear in any other blocks
    //   3. Create a mining reward transaction
    //   4. Perform proof of work
    //   5. Add block to blockchain
    //
    // Currently the proof of work consists of finding a hash begining with 4 0's.
    // This is slightly similar to how bitcoin uses a target to find a smaller hash
    // but seems much easier to implement. In the future it might be better to switch
    // back to a byte array and implement an increasing difficulty target.
    pub fn mine_block(&mut self, reward_address: &str) -> Result<(), &str> {
        let mut transactions: Vec<SignedTransaction> = Vec::new();
        // TODO: Replace with longest chain (aka highest id)
        let latest_block = self.blocks.last().expect("Unknown latest block");
        let new_index = latest_block.index + 1;
        let previous_hash = &latest_block.hash;

        for _ in 0..Block::MAX_TRANSACTIONS {
            let transaction_result = self.pending_transactions.get_mut().pop();
            if transaction_result.is_none() {
                // no transactions pending
                break;
            }
            let transaction = transaction_result.unwrap();

            if !(transaction.is_valid()) {
                continue;
            }

            if self.calculate_balance(&transaction.transaction.from) < transaction.transaction.amount {
                continue
            }

            // TODO: get rid of this O(n^2) bit
            for t in transactions.iter() {
                if transaction.hash == t.hash {
                    continue;
                }
            }

            // If found, skip this transaction
            match self.find_transaction_by_hash(&transaction.hash) {
                Ok(_) => continue,
                Err(_) => transactions.push(transaction)
            }
        }

        if transactions.len() <= 0 {
            return Err("No transactions found");
        }

        let reward_transaction = SignedTransaction::create_reward(reward_address);
        transactions.push(reward_transaction);

        let mut block = Block::create(new_index, transactions, previous_hash, 0);
        // TODO: This probably could be done with 2 128 byte ints or a byte array
        // TODO: change target to decrease after x # of blocks making proof harder
        while &block.hash[0..4] != "0000" {
            block.update_nonce(block.nonce + 1)
        }

        self.blocks.push(block);
        // TODO: setup increasing reward/decreasing difficulty
        Ok(())
    }

    // A really inefficent way to find a block by a particular index
    fn find_block_by_index(&self, index: u64) -> Result<&Block, &str> {
        // TODO: Not even sure if this method is needed, 
        // if so no need to iterate through each block
        for block in self.blocks.iter() {
            if block.index == index {
                return Ok(&block);
            }
        }

        Err("Unable to find block")
    }

    // A really inefficent way to find a block by a particular hash (currently unused)
    // fn find_block_by_hash(&self, hash: &str) -> Result<&Block, String> {
    //     // TODO: Not even sure if this method is needed, 
    //     // if so could be replaced with a merkle tree
    //     for block in self.blocks.iter() {
    //         if block.hash == hash {
    //             return Ok(&block);
    //         }
    //     }

    //     Err("Unable to find block".to_string())
    // }

    // Finds a transaction in the ledget with a given hash
    //
    // TODO: Might be able to implement some lookup table or merkle tree
    fn find_transaction_by_hash(&self, hash: &str) -> Result<&SignedTransaction, String> {
        for block in self.blocks.iter() {
            for transaction in block.transactions.iter() {
                if transaction.hash == hash {
                    return Ok(&transaction);
                }
            }
        }
        Err("No transaction found".to_string())
    }
}
