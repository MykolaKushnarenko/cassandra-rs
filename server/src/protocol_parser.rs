//! Serialization and deserialization of messages.
//!
//! This module provides the `ProtocolParser` for converting messages between bytes and
//! structured types using JSON, with a custom end-of-stream marker.

use std::marker::PhantomData;
use serde::{Deserialize, Serialize};
use crate::error::{AppResult, Error};

/// The byte used to mark the end of a message in the stream.
const END_OF_STREAM: u8 = 0u8;

/// A parser for the application's communication protocol.
///
/// It handles JSON serialization and deserialization and manages the `END_OF_STREAM` marker.
pub(crate) struct ProtocolParser<T> {
    _marker: PhantomData<T>,
}

impl<T: for<'de> Deserialize<'de>> ProtocolParser<T> {
    /// Deserializes a message from a byte slice.
    ///
    /// It expects the byte slice to end with an `END_OF_STREAM` marker, which it removes
    /// before parsing the remainder as JSON.
    pub(crate) fn deserialize(request: &[u8]) -> AppResult<T> {
        let normalized_request = &request[..request.len() - 1]; // removing 0u8 at the end

        let deserialize_result: Result<T, _> = serde_json::from_slice(normalized_request);

        match deserialize_result {
            Ok(value) => Ok(value),
            Err(_) => Err(Error::ParseError)
        }
    }
}

impl<T: Serialize> ProtocolParser<T> {
    /// Serializes a message into a byte vector.
    ///
    /// The message is serialized to JSON and an `END_OF_STREAM` marker is appended to the result.
    pub(crate) fn serialize(response: &T) -> AppResult<Vec<u8>> {
        let deserialize_result = serde_json::to_vec(response);

        match deserialize_result {
            Ok(mut bytes) => {
                bytes.push(END_OF_STREAM);
                Ok(bytes)
            },
            Err(_) => { Err(Error::ParseError) }
        }
    }
}