use alloy::primitives::{bytes::Buf, Bytes};
use anyhow::Result;

pub fn parse_request_template(bytes: Bytes) -> Result<Bytes> {
    let mut bytes = bytes;

    let version = bytes.get_u8();

    if version != 1 {
        return Err(anyhow::anyhow!("unsupported version"));
    }

    Ok(bytes)
}
