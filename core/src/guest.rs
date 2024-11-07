use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GuestInputRequest {
    pub url: String,
    pub data: Vec<u8>,
    pub server_name: String,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GuestInputResponse {
    pub time: String,
    pub stream: Vec<u8>,
    pub random: Vec<u8>,
    pub response: Vec<u8>,
    #[serde(default)]
    pub filtered_responses: Vec<FilteredResponse>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GuestInput {
    pub request: GuestInputRequest,
    pub response: GuestInputResponse,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GuestOutput {
    pub response_data: Vec<u8>,
    pub request_hash: [u8; 32],
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FilteredResponse {
    pub begin: usize,
    pub length: usize,

    pub content: Vec<u8>,
}
