#![cfg(test)]

use shared::protocol::frame::Frame;
use shared::protocol::protocol_writer::ProtocolWriter;
use shared::protocol::types::*;
use std::io::Cursor;

#[test]
fn test_send_request() {
    let cursor = Cursor::new(Vec::<u8>::new());
    let mut protocol_writer = ProtocolWriter::new(cursor);

    let request = Request::Add(Entry {
        value: "Test data".to_string(),
        replication_factor: None,
    });
    let write_op_result = protocol_writer.send_request(&request);

    assert!(write_op_result.is_ok());

    let mut cursor = protocol_writer.into_inner().unwrap();
    cursor.set_position(0);
    let body = Frame::decode(&mut cursor).unwrap().body;

    let request: Request = bincode::deserialize(&body).unwrap();

    assert_eq!(
        request,
        Request::Add(Entry {
            value: "Test data".to_string(),
            replication_factor: None
        })
    );
}
