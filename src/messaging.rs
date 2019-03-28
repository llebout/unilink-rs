use std::io::{Read, Write};

pub trait MessageWriter {
    fn write_message(&mut self, message: &[u8]) -> Result<(), ()>;
}

pub trait MessageReader {
    fn read_message(&mut self) -> Result<Vec<u8>, ()>;
}

impl<T: Read> MessageReader for T {
    fn read_message(&mut self) -> Result<Vec<u8>, ()> {
        let mut len_buf = [0u8; 2];

        self.read_exact(&mut len_buf).map_err(|_| {})?;

        let mut buf = vec![0u8; u16::from_be_bytes(len_buf) as usize];

        self.read_exact(&mut buf).map_err(|_| {})?;

        Ok(buf)
    }
}

impl<T: Write> MessageWriter for T {
    fn write_message(&mut self, message: &[u8]) -> Result<(), ()> {
        let len = message.len() as u16;

        let len_buf = len.to_be_bytes();

        self.write_all(&len_buf).map_err(|_| {})?;
        self.write_all(message).map_err(|_| {})?;

        Ok(())
    }
}