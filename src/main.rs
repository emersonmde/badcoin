mod kepair;

use secp256k1::{Secp256k1, Message};

fn main() {

    let secp = Secp256k1::new();
    let keypair = kepair::Keypair::new();
    let secret_key = keypair.secret_key();
    let public_key = keypair.public_key();
    let message = Message::from_slice(&[0xab; 32]).expect("32 bytes");
    // println!("{:?}", keypair.export_secret_key());
    let a = keypair.export_secret_key();
    let b = keypair.export_public_key();
    println!("{:?}", a);
    println!("{:?}", b);
    let c = kepair::Keypair::new_from_key(&a, &b);
    println!("{:?}", c.secret_key());
    println!("{:?}", c.public_key());

    println!("{:?}", secret_key);
    println!("{:?}", public_key);
    // println!("{:?}", message);

    let sig = secp.sign(&message, &secret_key);
    // println!("{:?}", sig);

    assert!(secp.verify(&message, &sig, &public_key).is_ok());
}
