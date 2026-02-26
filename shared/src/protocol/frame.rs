//! Frame management for the protocol 
//!
//! This represents a frame in the protocol, which is used to read and write data to and from

pub use crate::protocol::types::{Flags, Opcode, Version};
use std::io::Read;

#[derive(Debug, Clone)]
pub struct Frame {
    pub version: Version,
    pub flags: Flags,
    pub stream: u16,
    pub opcode: Opcode,
    pub length: u32,
    pub body: Vec<u8>
}

impl Frame {
    /// Creates a new frame with the given parameters.
    /// 
    /// Flags and stream are optional, defaulting to None and 0 respectively.
    pub fn new(version: Version, flags: Option<Flags>, stream: Option<u16>, opcode: Opcode, length: u32, body: Vec<u8>) -> Self {
        let flags = flags.unwrap_or(Flags::None);
        let stream = stream.unwrap_or(0);

        Self { version, flags, stream, opcode, length, body }
    }

    /// Encodes the frame into a vector of bytes.
    pub fn encode(self) -> Result<Vec<u8>, std::io::Error> {
        let mut bytes = Vec::with_capacity(9 + self.body.len());
        bytes.push(self.version as u8);
        bytes.push(self.flags as u8);
        bytes.extend_from_slice(&self.stream.to_be_bytes());
        bytes.push(self.opcode as u8);
        bytes.extend_from_slice(&self.length.to_be_bytes());
        bytes.extend_from_slice(&self.body);
        Ok(bytes)
    }

    /// Decodes a frame from a stream.
    pub fn decode(stream: &mut impl Read) -> Result<Frame, std::io::Error> {
        let mut version_byte = [0u8; 1];
        stream.read_exact(&mut version_byte)?;
        let version = Version::from(version_byte[0]);

        let mut flags_byte = [0u8; 1];
        stream.read_exact(&mut flags_byte)?;
        let flags = Flags::from(flags_byte[0]);

        let mut stream_bytes = [0u8; 2];
        stream.read_exact(&mut stream_bytes)?;
        let stream_id = u16::from_be_bytes(stream_bytes);

        let mut opcode_byte = [0u8; 1];
        stream.read_exact(&mut opcode_byte)?;
        let opcode = Opcode::from(opcode_byte[0]);

        let mut length_bytes = [0u8; 4];
        stream.read_exact(&mut length_bytes)?;
        let length = u32::from_be_bytes(length_bytes);

        let mut body = vec![0u8; length as usize];
        stream.read_exact(&mut body)?;

        Ok(Frame {
            version,
            flags,
            stream: stream_id,
            opcode,
            length,
            body,
        })
    }
}