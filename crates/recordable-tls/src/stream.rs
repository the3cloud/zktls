use std::{
    fs::File,
    io::{Read, Result, Write},
};

pub struct RecordableStream<T: Read + Write> {
    inner: T,
    record_file: File,
}

impl<T: Read + Write> RecordableStream<T> {
    pub fn new(inner: T, file: File) -> Self
    where
        Self: Sized,
    {
        RecordableStream {
            inner,
            record_file: file,
        }
    }
}

impl<T: Read + Write> Read for RecordableStream<T> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        let bytes_read = self.inner.read(buf)?;
        if bytes_read > 0 {
            let length_bytes = (bytes_read as u32).to_be_bytes();
            self.record_file.write_all(&[0x01])?;
            self.record_file.write_all(&length_bytes)?;
            self.record_file.write_all(&buf[..bytes_read])?;
        }
        Ok(bytes_read)
    }
}

impl<T: Read + Write> Write for RecordableStream<T> {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        let bytes_written = self.inner.write(buf)?;
        if bytes_written > 0 {
            let length_bytes = (bytes_written as u32).to_be_bytes();
            self.record_file.write_all(&[0x02])?;
            self.record_file.write_all(&length_bytes)?;
            self.record_file.write_all(&buf[..bytes_written])?;
        }
        Ok(bytes_written)
    }

    fn flush(&mut self) -> Result<()> {
        self.inner.flush()?;
        self.record_file.flush()
    }
}
