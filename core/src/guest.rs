use alloy::{
    primitives::{keccak256, Bytes, B256},
    sol_types::SolValue,
};
use serde::{Deserialize, Serialize};

use crate::prove_request::Request;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GuestInputRequest {
    pub url: String,
    pub server_name: String,
    pub request: Request,
    pub encrypted_key: Bytes,
}

impl GuestInputRequest {
    pub fn request_hash(&self) -> B256 {
        let res = compute_request_hash(
            self.url.clone(),
            self.server_name.clone(),
            self.encrypted_key.clone(),
            self.request.clone(),
        );

        let res: Bytes = res.into();

        println!("res: {:?}", res);

        keccak256(res)
    }
}

fn compute_template_request_hash(
    remote: String,
    server_name: String,
    encrypted_key: Bytes,
    template_hash: B256,
    offsets: &[u64],
    fields: &[Bytes],
) -> Vec<u8> {
    let data = (
        remote,
        server_name,
        encrypted_key,
        template_hash,
        offsets,
        fields,
    );

    data.abi_encode_sequence()
}

fn compute_request_hash(
    remote: String,
    server_name: String,
    encrypted_key: Bytes,
    request: Request,
) -> Vec<u8> {
    match request {
        Request::Template(template_request) => compute_template_request_hash(
            remote,
            server_name,
            encrypted_key,
            template_request.template_hash,
            &template_request.offsets,
            &template_request.fields,
        ),
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GuestInputResponse {
    pub time: String,
    pub stream: Vec<u8>,
    pub random: Vec<u8>,
    pub response: Vec<u8>,
    #[serde(default)]
    pub filtered_responses_begin: Vec<u64>,
    #[serde(default)]
    pub filtered_responses_length: Vec<u64>,
    #[serde(default)]
    pub filtered_responses: Vec<Vec<u8>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GuestInput {
    pub request: GuestInputRequest,
    pub response: GuestInputResponse,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GuestOutput {
    pub response_data: Vec<u8>,
    pub request_hash: [u8; 32],
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FilteredResponse {
    pub begin: u64,
    pub length: u64,

    pub content: Vec<u8>,
}
