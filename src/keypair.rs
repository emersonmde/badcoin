extern crate serde;

use rand::rngs::OsRng;
use secp256k1::{Secp256k1, SecretKey, PublicKey, Message, Signature};
use std::str::FromStr;
use serde::ser::Serialize;

pub struct Keypair {
    public_key: PublicKey,
    secret_key: SecretKey
}

impl Keypair {
    pub fn new() -> Keypair {
        let secp = Secp256k1::new();
        let mut rng = OsRng::new().expect("OsRng");
        let (secret_key, public_key) = secp.generate_keypair(&mut rng);
        Keypair {
            public_key: public_key,
            secret_key: secret_key
        }
    }

    // pub fn new_from_key(secret_key: String) -> Keypair {
    //     let secp = Secp256k1::new();
    //     let key = SecretKey::from_str(&secret_key).expect("failed to parse secret key");
    //     let public_key = PublicKey::from_secret_key(&secp, &key);
    //     Keypair {
    //         public_key: public_key,
    //         secret_key: key
    //     }
    // }

    pub fn new_from_slice(secret_key: &[u8]) -> Keypair {
        let secp = Secp256k1::new();
        let key = SecretKey::from_slice(secret_key).expect("Failed to parse secret key");
        let public_key = PublicKey::from_secret_key(&secp, &key);
        Keypair {
            public_key: public_key,
            secret_key: key
        }
    }

    pub fn public_key(&self) -> PublicKey {
        self.public_key
    }

    pub fn secret_key(&self) -> SecretKey {
        self.secret_key
    }

    pub fn export_public_key(&self) -> String {
        self.public_key.to_string()
    }

    // TODO: Set change message to hash
    pub fn sign(&self, message: &[u8]) -> String {
        let secp = Secp256k1::new();
        let message_bytes = Message::from_slice(&message).expect("Unable to read Message");
        let signature = secp.sign(&message_bytes, &self.secret_key);
        signature.to_string()
    }

    // TODO: Set change message to hash
    pub fn verify(&self, signature: String, message: &[u8], public_key: Option<String>) -> bool {
        let secp = Secp256k1::new();
        let m = Message::from_slice(&message).expect("Unable to read Message");
        let s = Signature::from_str(&signature).expect("Unable to read Signature");
        match public_key {
            None => secp.verify(&m, &s, &self.public_key).is_ok(),
            Some(key) => {
                let pk = PublicKey::from_str(&key).expect("Unable to read Public Key");
                secp.verify(&m, &s, &pk).is_ok()
            }
        }
    }

}

pub fn verify_signature(public_key: &str, signature: &str, message: &[u8]) -> bool {
        let secp = Secp256k1::new();
        let k = PublicKey::from_str(public_key).expect("Unable to read Public Key");
        let m = Message::from_slice(message).expect("Unable to read Message");
        let s = Signature::from_str(signature).expect("Unable to read Signature");
        secp.verify(&m, &s, &k).is_ok()
        

}