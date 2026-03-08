//! Wrapper over TCP stream for encoding data into protocol frames and write to the stream

use crate::protocol::frame::Frame;
use crate::protocol::types::{Opcode, Request, Response, Version};
use std::io::{BufWriter, Write};

pub struct ProtocolWriter<T: Write> {
    writer: BufWriter<T>,
}

impl<'a, T: Write> ProtocolWriter<T> {
    pub fn new(writer: T) -> Self {
        let writer = BufWriter::new(writer);
        Self { writer }
    }

    pub fn send_request(&mut self, request: &Request) -> Result<(), std::io::Error> {
        let request_bytes = bincode::serialize(request).unwrap();

        let frame = Frame::new(
            Version::Request,
            None,
            None,
            Opcode::Query,
            request_bytes.len() as u32,
            request_bytes,
        );
        let frame_bytes = frame.encode()?;
        self.writer.write_all(&frame_bytes)?;
        self.writer.flush()?;

        Ok(())
    }

    pub fn send_response(&mut self, request: &Response) -> Result<(), std::io::Error> {
        let request_bytes = bincode::serialize(request).unwrap();

        let frame = Frame::new(
            Version::Response,
            None,
            None,
            Opcode::Query,
            request_bytes.len() as u32,
            request_bytes,
        );
        let frame_bytes = frame.encode()?;
        self.writer.write_all(&frame_bytes)?;
        self.writer.flush()?;

        Ok(())
    }

    pub fn into_inner(self) -> Result<T, std::io::Error> {
        self.writer.into_inner().map_err(|e| e.into_error())
    }
}
