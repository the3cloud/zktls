use std::io::{Read, Result, Write};

enum TypedData {
    Incoming(Vec<u8>),
    Outgoing(Vec<u8>),
}

pub struct ReplayStream {
    replay_data: Vec<TypedData>,
    offset: usize,
}

impl ReplayStream {
    pub fn new(data: Vec<u8>) -> Self {
        let mut offset = 0;
        let mut replay_data = Vec::new();

        while offset < data.len() {
            let forward = data[offset];
            offset += 1;

            let length = u32::from_be_bytes(data[offset..offset + 4].try_into().unwrap());
            offset += 4;

            let data = data[offset..offset + length as usize].to_vec();
            offset += length as usize;

            match forward {
                1 => replay_data.push(TypedData::Incoming(data)),
                2 => replay_data.push(TypedData::Outgoing(data)),
                _ => panic!("Invalid forward value"),
            }
        }

        ReplayStream {
            replay_data,
            offset: 0,
        }
    }
}

impl Read for ReplayStream {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        let data = &self.replay_data[self.offset];

        match data {
            TypedData::Incoming(data) => {
                let length = data.len();

                buf[..length].copy_from_slice(data);

                self.offset += 1;
                Ok(length)
            }
            TypedData::Outgoing(_data) => {
                panic!("Outgoing data not supported");
            }
        }
    }
}

impl Write for ReplayStream {
    fn write(&mut self, _buf: &[u8]) -> Result<usize> {
        let data = &self.replay_data[self.offset];

        match data {
            TypedData::Outgoing(data) => {
                let length = data.len();
                self.offset += 1;
                Ok(length)
            }
            TypedData::Incoming(_data) => {
                panic!("Incoming data not supported");
            }
        }
    }

    fn flush(&mut self) -> Result<()> {
        Ok(())
    }
}
