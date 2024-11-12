use std::io::{Read, Result, Write};

use t3zktls_core::TypedData;

pub struct ReplayStream {
    replay_data: Vec<TypedData>,
    offset: usize,
}

impl ReplayStream {
    pub fn new(data: Vec<u8>) -> Self {
        let mut offset = 0;
        let mut replay_data = Vec::new();

        while offset < data.len() {
            let typed_data = TypedData::from_bytes(&data[offset..]).unwrap();

            offset += typed_data.length();

            replay_data.push(typed_data);
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
