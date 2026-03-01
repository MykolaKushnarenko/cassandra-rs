//! Wrapper over TCP stream for reading and decoding protocol frames

use crate::protocol::frame::Frame;
use crate::protocol::types::{Request, Response};
use io::Read;
use std::io;
use std::io::BufReader;

pub struct ProtocolReader<T: Read> {
    reader: BufReader<T>,
}

impl<T: Read> ProtocolReader<T> {
    pub fn new(reader: T) -> Self {
        let reader = BufReader::new(reader);
        Self { reader }
    }

    pub fn receive_response(&mut self) -> Result<Response, io::Error> {
        let frame = Frame::decode(&mut self.reader)?;
        let response: Response = bincode::deserialize(&frame.body)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
        Ok(response)
    }

    pub fn receive_request(&mut self) -> Result<Request, io::Error> {
        let frame = Frame::decode(&mut self.reader)?;
        let response: Request = bincode::deserialize(&frame.body)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

        Ok(response)
    }
}
