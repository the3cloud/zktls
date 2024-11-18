use std::collections::BTreeMap;

use alloy::primitives::{
    bytes::{BufMut, BytesMut},
    Bytes, B256,
};
use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplateRequest {
    pub template_hash: B256,
    pub template: Bytes,
    pub offsets: Vec<u64>,
    pub fields: Vec<Bytes>,
    pub unencrypted_offset: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Request {
    Template(TemplateRequest),
}

impl Request {
    pub fn data(&self) -> Result<Bytes> {
        match self {
            Request::Template(req) => {
                let template = &req.template;

                let mut ordered_fields = BTreeMap::new();
                for (idx, field) in req.offsets.iter().zip(&req.fields) {
                    ordered_fields.insert(*idx, field);
                }

                let mut offset = 0;

                let mut bytes = BytesMut::new();
                for (idx, field) in ordered_fields {
                    let append_bytes = template
                        .get(offset..idx as usize)
                        .ok_or(anyhow!("index out of bounds in template fields"))?;

                    bytes.put_slice(append_bytes);
                    bytes.put_slice(field);

                    offset = idx as usize;
                }

                bytes.put_slice(
                    template
                        .get(offset..)
                        .ok_or(anyhow!("index out of bounds in template"))?,
                );
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
    pub encrypted_key: Bytes,

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
