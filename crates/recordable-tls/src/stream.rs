use std::{
    fs::{File, OpenOptions},
    io::{Read, Result, Write},
};

pub struct RecordableStream<T: Read + Write> {
    inner: T,
    record_file: File,
}

impl<T: Read + Write> RecordableStream<T> {
    pub fn new(inner: T) -> Self
    where
        Self: Sized,
    {
        if std::path::Path::new("target/stream").exists() {
            std::fs::remove_file("target/stream").unwrap();
        }

        let record_file = OpenOptions::new()
            .create(true)
            .append(true)
            .open("target/stream")
            .unwrap();

        RecordableStream { inner, record_file }
    }
}

impl<T: Read + Write> Read for RecordableStream<T> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        let bytes_read = self.inner.read(buf)?;
        if bytes_read > 0 {
            let hex_data = hex::encode(&buf[..bytes_read]);
            writeln!(self.record_file, "<{}", hex_data)?;
        }
        Ok(bytes_read)
    }
}

impl<T: Read + Write> Write for RecordableStream<T> {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        let bytes_written = self.inner.write(buf)?;
        if bytes_written > 0 {
            let hex_data = hex::encode(&buf[..bytes_written]);
            writeln!(self.record_file, ">{}", hex_data)?;
        }
        Ok(bytes_written)
    }

    fn flush(&mut self) -> Result<()> {
        self.inner.flush()?;
        self.record_file.flush()
    }
}
