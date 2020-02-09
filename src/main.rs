extern crate badcoin;
use badcoin::wallet::Wallet;
use badcoin::blockchain::Blockchain;

fn main() {
    let wallet = Wallet::new();
    wallet.send("asdf", 10);

    let mut bc = Blockchain::new();
    bc.mine_block("sadaf");
}
