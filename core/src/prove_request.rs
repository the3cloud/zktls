use alloy::primitives::{Bytes, B256};
use serde::{Deserialize, Serialize};

use crate::GuestInputRequest;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ProveRequest {
    pub request_id: B256,
    pub prover_id: B256,

    pub remote: GuestInputRequest,

    pub request_data: Bytes,
    pub response_template_id: B256,
    pub response_template: Bytes,

    pub max_response_size: u64,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ProverResponse {
    pub request_id: B256,
    pub prover_id: B256,

    pub response_data: Bytes,
    pub request_hash: B256,
}
