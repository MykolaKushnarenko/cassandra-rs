//! Connection management for the server.
//!
//! This module provides the `Connection` struct which handles reading from and writing to
//! a TCP stream using a custom protocol.

use crate::error::{AppResult, Error};
use crate::protocol::protocol_reader::ProtocolReader;
use crate::protocol::protocol_writer::ProtocolWriter;
use crate::protocol::types::{Request, Response};
use std::net::TcpStream;

/// Represents a connection to a client.
///
/// It uses buffered readers and writers for efficient I/O operations over a `TcpStream`.
pub struct Connection {
    stream: TcpStream,
}

impl Connection {
    /// Creates a new `Connection` from a `TcpStream`.
    pub fn new(stream: TcpStream) -> Self {
        Self { stream }
    }

    /// Receives a request from the client.
    pub fn receive_request(&mut self) -> AppResult<Request> {
        let mut reader = ProtocolReader::new(&self.stream);
        reader
            .receive_request()
            .map_err(|err| Error::ConnectionError(Some(err)))
    }

    /// Receives a response from the server.
    pub fn receive_response(&mut self) -> AppResult<Response> {
        let mut reader = ProtocolReader::new(&self.stream);
        reader
            .receive_response()
            .map_err(|err| Error::ConnectionError(Some(err)))
    }

    /// Sends a response to the client.
    pub fn send_response(&mut self, value: &Response) -> AppResult<()> {
        let mut writer = ProtocolWriter::new(&self.stream);
        writer
            .send_response(value)
            .map_err(|err| Error::ConnectionError(Some(err)))
    }

    /// Sends a request to the server and waits for a response.
    pub fn send_request_with_response(&mut self, request: &Request) -> AppResult<Response> {
        {
            let mut writer = ProtocolWriter::new(&self.stream);
            writer
                .send_request(request)
                .map_err(|err| Error::ConnectionError(Some(err)))?;
        }
        self.receive_response()
    }
}
