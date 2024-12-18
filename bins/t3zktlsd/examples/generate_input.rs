use std::env;

use alloy::{hex::FromHex, primitives::B256, signers::local::LocalSigner};

fn main() {
    let private_key_str = env::var("TEST_PRIVATE_KEY").unwrap();
    let private_key = B256::from_hex(&private_key_str).unwrap();

    let signer = LocalSigner::from_bytes(&private_key);
}
