use rand::rngs::OsRng;
use secp256k1::{Secp256k1, SecretKey, PublicKey};
use std::str::FromStr;

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

    pub fn new_from_key(secret_key: &str, public_key: &str) -> Keypair {
        Keypair {
            public_key: PublicKey::from_str(public_key).expect("failed to parse pulic key"),
            secret_key: SecretKey::from_str(secret_key).expect("failed to parse secret key")
        }
    }

    pub fn public_key(&self) -> PublicKey {
        self.public_key
    }

    pub fn secret_key(&self) -> SecretKey {
        self.secret_key
    }

    pub fn export_secret_key(&self) -> String {
        self.secret_key.to_string()
    }

    pub fn export_public_key(&self) -> String {
        self.public_key.to_string()
    }
}