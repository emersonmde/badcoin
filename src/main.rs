extern crate badcoin;
use badcoin::wallet::Wallet;

fn main() {
    let wallet = Wallet::new();
    wallet.send("asdf", 10);
}
