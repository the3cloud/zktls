use alloy::primitives::{bytes::Buf, Bytes};
use anyhow::Result;

pub struct ResponseTemplate {
    regex: String,
}

impl ResponseTemplate {
    pub fn new(bytes: Bytes) -> Result<Self> {
        let mut bytes = bytes;

        let version = bytes.get_u8();

        if version != 1 {
            return Err(anyhow::anyhow!("unsupported version"));
        }

        let mode = bytes.get_u8();

        if mode == 1 {
            let length = bytes.get_u16() as usize;
            let s = String::from_utf8(bytes.slice(..length).to_vec())?;

            Ok(Self { regex: s })
        } else {
            Err(anyhow::anyhow!("unsupported mode"))
        }
    }

    pub fn regex(&self) -> &str {
        &self.regex
    }
}
