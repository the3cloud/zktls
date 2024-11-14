use std::collections::HashMap;

use alloy::primitives::{
    bytes::{BufMut, BytesMut},
    Bytes, B256,
};
use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OriginalRequest {
    pub data: Vec<u8>,
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

impl Request {
    pub fn new_original(data: Vec<u8>) -> Self {
        Self::Original(OriginalRequest { data })
    }
}

impl Request {
    pub fn data(&self) -> Result<Bytes> {
        match self {
            Request::Original(req) => Ok(req.data.clone().into()),
            Request::Template(req) => {
                let template = req.template.clone();

                let mut bytes = BytesMut::new();
                for (idx, field) in &req.fields {
                    let append_bytes = template
                        .get(..*idx as usize)
                        .ok_or(anyhow!("index out of bounds"))?;

                    bytes.put_slice(append_bytes);
                    bytes.put_slice(field);
                }
                Ok(bytes.freeze().into())
            }
        }
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ResponseTemplate {
    None,
    Position { begin: u64, length: u64 },
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
    pub response_template: ResponseTemplate,

    pub max_response_size: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProveResponse {
    pub request_id: B256,

    pub response_data: Bytes,
    pub request_hash: B256,

    pub proof: Bytes,
}
