//! Connection management for the server.
//!
//! This module provides the `Connection` struct which handles reading from and writing to
//! a TCP stream using a custom protocol.

use crate::error::{AppResult, Error};
use crate::protocol_parser::ProtocolParser;
use serde::{Deserialize, Serialize};
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::net::TcpStream;

/// Represents a connection to a client.
///
/// It uses buffered readers and writers for efficient I/O operations over a `TcpStream`.
pub(crate) struct Connection<'a> {
    reader: BufReader<&'a TcpStream>,
    writer: BufWriter<&'a TcpStream>,
}

impl<'a> Connection<'a> {
    /// Creates a new `Connection` from a `TcpStream`.
    pub(crate) fn new(stream: &'a TcpStream) -> Self {
        let reader = BufReader::new(stream);
        let writer = BufWriter::new(stream);

        Self {
            reader,
            writer
        }
    }
    
    /// Receives and deserializes a message of type `T` from the connection.
    ///
    /// This method reads from the stream until a null byte (`0u8`) is encountered,
    /// and then attempts to deserialize the collected bytes.
    pub(crate) fn receive<T: for<'de> Deserialize<'de>>(&mut self) -> AppResult<T>{
        let mut buffer = Vec::new();
        
        let read_op_result = self.reader.read_until(0u8, &mut buffer);
        match read_op_result {
            Ok(0) => {return Err(Error::ConnectionError(None))},
            Ok(_) => {},
            Err(err) => return Err(Error::ConnectionError(Some(err)))
        }
        
        let result: T = ProtocolParser::deserialize(&buffer)?;

        Ok(result)
    }

    /// Serializes and sends a message of type `T` to the connection.
    ///
    /// The message is serialized using the `ProtocolParser` and then written
    /// to the underlying stream followed by a flush operation.
    pub(crate) fn send<T: Serialize>(&mut self, value: &T) -> AppResult<()>{
        let result = ProtocolParser::serialize(&value)?;

        let write_op_result = self.writer.write_all(&result);
        let flush_op_result = self.writer.flush();

        match (write_op_result, flush_op_result) {
            (Ok(_), Ok(_)) => Ok(()),
            (Err(err), _) => Err(Error::ConnectionError(Some(err))),
            (_, Err(err)) => Err(Error::ConnectionError(Some(err)))
        }
    }
}