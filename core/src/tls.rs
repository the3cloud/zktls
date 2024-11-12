use anyhow::Result;

pub enum TypedData {
    Incoming(Vec<u8>),
    Outgoing(Vec<u8>),
}

impl TypedData {
    pub fn new_incoming(data: Vec<u8>) -> Self {
        Self::Incoming(data)
    }

    pub fn new_outgoing(data: Vec<u8>) -> Self {
        Self::Outgoing(data)
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<Self> {
        let mut offset = 0;

        let forward = bytes[offset];
        offset += 1;

        let length = u32::from_be_bytes(bytes[offset..offset + 4].try_into()?) as usize;
        offset += 4;

        log::debug!("forward: {}, length: {}", forward, length);

        if forward == 1 {
            Ok(Self::Incoming(bytes[offset..offset + length].to_vec()))
        } else if forward == 2 {
            Ok(Self::Outgoing(bytes[offset..offset + length].to_vec()))
        } else {
            Err(anyhow::anyhow!("Invalid forward value"))
        }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        match self {
            Self::Incoming(data) => {
                let mut bytes = vec![1];
                bytes.extend_from_slice(&data.len().to_be_bytes());
                bytes.extend_from_slice(data);
                bytes
            }
            Self::Outgoing(data) => {
                let mut bytes = vec![2];
                bytes.extend_from_slice(&data.len().to_be_bytes());
                bytes.extend_from_slice(data);
                bytes
            }
        }
    }

    pub fn length(&self) -> usize {
        match self {
            Self::Incoming(data) => data.len() + 5,
            Self::Outgoing(data) => data.len() + 5,
        }
    }
}
