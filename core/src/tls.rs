use anyhow::Result;

#[derive(Debug, PartialEq, Eq)]
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
        let forward = bytes[0];

        let length_bytes = bytes[1..5].try_into()?;

        let length = u32::from_be_bytes(length_bytes) as usize;

        log::debug!("forward: {}, length: {}", forward, length);

        if forward == 1 {
            Ok(Self::Incoming(bytes[5..5 + length].to_vec()))
        } else if forward == 2 {
            Ok(Self::Outgoing(bytes[5..5 + length].to_vec()))
        } else {
            Err(anyhow::anyhow!("Invalid forward value"))
        }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        match self {
            Self::Incoming(data) => {
                let length = data.len() as u32;

                let mut bytes = vec![1];
                bytes.extend_from_slice(&length.to_be_bytes());
                bytes.extend_from_slice(data);
                bytes
            }
            Self::Outgoing(data) => {
                
                let length = data.len() as u32;

                let mut bytes = vec![2];
                bytes.extend_from_slice(&length.to_be_bytes());
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

#[cfg(test)]
mod tests {
    use crate::TypedData;

    #[test]
    fn test_typed_data() {
        let _ = env_logger::builder().is_test(true).try_init();

        let typed_data = TypedData::new_incoming(vec![1, 2, 3, 4]);

        let encoded = typed_data.to_bytes();

        let decoded = TypedData::from_bytes(&encoded).unwrap();

        assert_eq!(typed_data, decoded);
    }
}
