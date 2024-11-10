use alloy::primitives::{bytes::Buf, Bytes};
use anyhow::Result;

use crate::Response;

pub fn parse_response_template(bytes: Bytes) -> Result<Vec<Response>> {
    let mut bytes = bytes;

    let version = bytes.get_u8();

    if version != 1 {
        return Err(anyhow::anyhow!("unsupported version"));
    }

    let mut responses = Vec::new();
    while bytes.remaining() > 0 {
        let response = build_response_template(&mut bytes)?;

        responses.push(response);
    }

    Ok(responses)
}

fn build_response_template(bytes: &mut Bytes) -> Result<Response> {
    let mode = bytes.get_u8();

    match mode {
        1 => {
            let begin = bytes.get_u64();
            let end = bytes.get_u64();

            Ok(Response::Position { begin, end })
        }
        2 => {
            let length = bytes.get_u16() as usize;
            let s = bytes
                .get(..length)
                .ok_or(anyhow::anyhow!("invalid length"))?
                .to_vec();

            Ok(Response::Regex(String::from_utf8(s)?))
        }
        3 => {
            let length = bytes.get_u16() as usize;
            let s = bytes
                .get(..length)
                .ok_or(anyhow::anyhow!("invalid length"))?
                .to_vec();

            Ok(Response::XPath(String::from_utf8(s)?))
        }
        4 => {
            let length = bytes.get_u16() as usize;
            let s = bytes
                .get(..length)
                .ok_or(anyhow::anyhow!("invalid length"))?
                .to_vec();

            Ok(Response::JsonPath(String::from_utf8(s)?))
        }
        _ => Err(anyhow::anyhow!("unsupported mode")),
    }
}
