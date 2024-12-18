use std::env;

use alloy::{
    hex::FromHex,
    primitives::{Address, FixedBytes, B256},
    signers::{local::LocalSigner, SignerSync},
};
use t3zktls_program_core::{Origin, Request, RequestClient, ResponseTemplate, Secp256k1Origin};

fn main() {
    let private_key_str = env::var("TEST_PRIVATE_KEY").unwrap();
    let private_key = B256::from_hex(&private_key_str).unwrap();

    let signer = LocalSigner::from_bytes(&private_key).unwrap();

    let request = b"GET /get HTTP/1.1\r\nHost: httpbin.org\r\nUser-Agent: curl/8.5.0\r\nAccept: */*\r\nConnection: Close\r\n\r\n";

    let response_template = vec![ResponseTemplate::Offset {
        begin: 9,
        length: 3,
    }];

    let origin = Origin::None;

    let mut request = Request {
        request: request.into(),
        remote_addr: "httpbin.org:443".into(),
        server_name: "httpbin.org".into(),
        response_template,
        client: RequestClient {
            client: Address::default(),
            max_gas_price: 1000000000000000000,
            max_gas_limit: 0,
        },
        origin,
    };

    let hash = request.request_hash();

    let signature = signer.sign_hash_sync(&hash).unwrap();
    let signature = signature.as_bytes();

    let origin = Secp256k1Origin {
        signature: FixedBytes::from(signature),
        nonce: 0,
    };

    request.origin = Origin::Secp256k1(origin);

    let s = serde_json::to_string(&request).unwrap();

    println!("{}", s);
}
