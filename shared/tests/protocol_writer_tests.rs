#![cfg(test)]

use std::io::Cursor;
use shared::protocol::frame::Frame;
use shared::protocol::types::*;
use shared::protocol::protocol_writer::{ProtocolWriter};

#[test]
fn test_send_request() {
    let cursor = Cursor::new(Vec::<u8>::new());
    let mut protocol_writer = ProtocolWriter::new(cursor);

    let request = Request::Add("Test data".to_string());
    let write_op_result = protocol_writer.send_request(&request);

    assert!(write_op_result.is_ok());

    let mut cursor = protocol_writer.into_inner().unwrap();
    cursor.set_position(0);
    let body = Frame::decode(&mut cursor).unwrap().body;

    let request: Request = bincode::deserialize(&body).unwrap();

    assert_eq!(request, Request::Add("Test data".to_string()));
}