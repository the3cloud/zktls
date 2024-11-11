use alloy::primitives::{bytes::Buf, Bytes};
use anyhow::Result;

use crate::ResponseTemplate;

pub fn parse_response_template(bytes: Bytes) -> Result<ResponseTemplate> {
    let mut bytes = bytes;

    let version = bytes.get_u8();

    if version != 1 {
        return Err(anyhow::anyhow!("unsupported version"));
    }

    let response = build_response_template(&mut bytes)?;

    Ok(response)
}

fn build_response_template(bytes: &mut Bytes) -> Result<ResponseTemplate> {
    let mode = bytes.get_u8();

    match mode {
        1 => {
            let begin = bytes.get_u64();
            let length = bytes.get_u64();

            Ok(ResponseTemplate::Position { begin, length })
        }
        2 => {
            let length = bytes.get_u16() as usize;
            let s = bytes
                .get(..length)
                .ok_or(anyhow::anyhow!("invalid length"))?
                .to_vec();

            Ok(ResponseTemplate::Regex(String::from_utf8(s)?))
        }
        3 => {
            let length = bytes.get_u16() as usize;
            let s = bytes
                .get(..length)
                .ok_or(anyhow::anyhow!("invalid length"))?
                .to_vec();

            Ok(ResponseTemplate::XPath(String::from_utf8(s)?))
        }
        4 => {
            let length = bytes.get_u16() as usize;
            let s = bytes
                .get(..length)
                .ok_or(anyhow::anyhow!("invalid length"))?
                .to_vec();

            Ok(ResponseTemplate::JsonPath(String::from_utf8(s)?))
        }
        _ => Err(anyhow::anyhow!("unsupported mode")),
    }
}
