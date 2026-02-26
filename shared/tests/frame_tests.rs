#![cfg(test)]

use io::Cursor;
use std::io;
use shared::protocol::frame::{Flags, Frame, Opcode, Version};

#[test]
fn new_should_create_frame() {
    let actual = vec![0u8; 1];
    let expected = actual.clone();

    let frame = Frame::new(Version::Request, None, None, Opcode::Query, actual.len() as u32, actual);

    assert_eq!(frame.version, Version::Request);
    assert_eq!(frame.flags, Flags::None);
    assert_eq!(frame.stream, 0);
    assert_eq!(frame.opcode, Opcode::Query);
    assert_eq!(frame.length, expected.len() as u32);
    assert_eq!(frame.body, expected);
}

#[test]
fn encode_should_return_bytes() {
    let body = vec![0u8; 1];

    let frame = Frame::new(Version::Request, Some(Flags::CustomPayload), Some(1234), Opcode::Query, body.len() as u32, body);
    let encoded_frame_result = frame.encode();

    assert_eq!(encoded_frame_result.is_ok(), true);

    let encoded_frame = encoded_frame_result.unwrap();
    assert_eq!(encoded_frame[0], 0x04);
    assert_eq!(encoded_frame[1], 0x04);
    assert_eq!(i16::from_be_bytes([encoded_frame[2], encoded_frame[3]]), 1234);
    assert_eq!(encoded_frame[4], 0x07);
    assert_eq!(u32::from_be_bytes([encoded_frame[5], encoded_frame[6], encoded_frame[7], encoded_frame[8]]), 1);
    assert_eq!(encoded_frame[9..10], vec![0u8; 1]);
}

#[test]
fn decode_should_return_bytes() {
    let body = "test data".as_bytes().to_vec();
    let expected_body = body.clone();
    let frame = Frame::new(Version::Request, Some(Flags::CustomPayload), Some(1234), Opcode::Query, body.len() as u32, body);
    let encoded_frame_result = frame.clone().encode();

    assert_eq!(encoded_frame_result.is_ok(), true);

    let mut cursor = Cursor::new(encoded_frame_result.unwrap());

    let decoded_frame_result = Frame::decode(&mut cursor);
    assert_eq!(decoded_frame_result.is_ok(), true);

    let decoded_frame = decoded_frame_result.unwrap();
    assert_eq!(decoded_frame.version, Version::Request);
    assert_eq!(decoded_frame.flags, Flags::CustomPayload);
    assert_eq!(decoded_frame.stream, 1234);
    assert_eq!(decoded_frame.opcode, Opcode::Query);
    assert_eq!(decoded_frame.length, expected_body.len() as u32);
    assert_eq!(decoded_frame.body, expected_body);
}