use alloy::primitives::{bytes::Buf, Bytes, B256};
use anyhow::Result;

#[derive(Debug, Clone)]
pub struct RequestTemplate {
    pub template: String,
}

impl RequestTemplate {
    pub fn new(bytes: Bytes) -> Result<Self> {
        let mut bytes = bytes;

        let version = bytes.get_u8();

        if version != 1 {
            return Err(anyhow::anyhow!("unsupported version"));
        }

        let template = bytes.slice(1..);

        Ok(Self {
            template: hex::encode(template),
        })
    }

    pub fn fill(&mut self, field: &B256, data: &[u8]) {
        let field_str = hex::encode(field);
        let data_str = hex::encode(data);

        self.template = self.template.replace(&field_str, &data_str);
    }

    pub fn finalize(self) -> Result<Bytes> {
        let result = hex::decode(&self.template)?;

        Ok(Bytes::from(result))
    }
}
