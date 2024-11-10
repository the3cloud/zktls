use std::collections::HashMap;

use alloy::primitives::{Bytes, B256};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OriginalRequest {
    pub data: Bytes,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplateRequest {
    pub template: Bytes,
    pub fields: HashMap<u64, Bytes>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Request {
    Original(OriginalRequest),
    Template(TemplateRequest),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Response {
    Position { begin: u64, end: u64 },
    Regex(String),
    XPath(String),
    JsonPath(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProveRequest {
    pub request_id: B256,
    pub prover_id: B256,

    pub remote: String,
    pub server_name: String,

    pub request: Request,
    pub response_template_id: B256,
    pub response_template: Vec<Response>,

    pub max_response_size: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProveResponse {
    pub request_id: B256,

    pub response_data: Bytes,
    pub request_hash: B256,

    pub proof: Bytes,
}
