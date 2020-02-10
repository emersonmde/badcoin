extern crate badcoin;
use badcoin::wallet::Wallet;
use badcoin::blockchain::Blockchain;
use badcoin::blockchain::SignedTransaction;
use badcoin::blockchain::Transaction;

fn main() {
    let wallet = Wallet::new();
    let public_key = wallet.keypair.export_public_key();
    wallet.send("asdf", 10);

    let transaction = Transaction::create("nobody", &public_key, 20);
    let signed_transaction = SignedTransaction::create(transaction, &wallet.keypair);

    let mut bc = Blockchain::new(&wallet.keypair);
    bc.add_pending_transaction(signed_transaction);

    let result = bc.mine_block("sadaf");

    match result {
        Ok(_) => println!("PROFIT"),
        Err(s) => println!("Err: {}", s)
    }
}
