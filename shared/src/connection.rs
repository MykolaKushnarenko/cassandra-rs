//! Connection management for the server.
//!
//! This module provides the `Connection` struct which handles reading from and writing to
//! a TCP stream using a custom protocol.

use std::net::TcpStream;
use crate::error::{AppResult, Error};
use crate::protocol::protocol_reader::ProtocolReader;
use crate::protocol::protocol_writer::ProtocolWriter;
use crate::protocol::types::{Request, Response};

/// Represents a connection to a client.
///
/// It uses buffered readers and writers for efficient I/O operations over a `TcpStream`.
pub struct Connection<'a> {
    reader: ProtocolReader<&'a TcpStream>,
    writer: ProtocolWriter<&'a TcpStream>,
}

impl<'a> Connection<'a> {
    /// Creates a new `Connection` from a `TcpStream`.
    pub fn new(stream: &'a TcpStream) -> Self {
        let reader = ProtocolReader::new(stream);
        let writer = ProtocolWriter::new(stream);

        Self {
            reader,
            writer
        }
    }
    
    /// Receives a request from the client.
    pub fn receive_request(&mut self) -> AppResult<Request>{
        let read_op_result = self.reader.receive_request();

        match read_op_result {
            Ok(_) => {},
            Err(err) => return Err(Error::ConnectionError(Some(err)))
        }

        Ok(read_op_result.unwrap())
    }

    /// Receives a response from the server.
    
    pub fn receive_response(&mut self) -> AppResult<Response>{
        let read_op_result = self.reader.receive_response();

        match read_op_result {
            Ok(_) => {},
            Err(err) => return Err(Error::ConnectionError(Some(err)))
        }

        Ok(read_op_result.unwrap())
    }

    /// Sends a response to the client.
    pub fn send_response(&mut self, value: Response) -> AppResult<()>{
        
        let write_op_result = self.writer.send_response(&value);

        match write_op_result {
            Ok(_) => Ok(()),
            Err(err) => Err(Error::ConnectionError(Some(err)))
        }
    }
    
    /// Sends a request to the client and waits for a response.
    pub fn send_request_with_response(&mut self, request: Request) -> AppResult<Response> {
        let write_op_result = self.writer.send_request(&request);
        
        match write_op_result {
            Ok(_) => {},
            Err(err) => return Err(Error::ConnectionError(Some(err)))
        }
        
        self.receive_response()
    }
}