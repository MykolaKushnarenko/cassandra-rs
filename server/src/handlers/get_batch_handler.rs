//! Handler for the "get batch" command.

use crate::storage::{GlobalStorage, Storage};
use shared::consistent_hash_ring::Range;
use shared::error::AppResult;
use shared::protocol::types::Response;

/// Gets a batch of values by the provided range of keys.
pub(crate) fn handle(ranges: &[Range], storage: &GlobalStorage) -> AppResult<Response> {
    let storage_guard = storage.lock().unwrap();

    let mut values = Vec::new();
    for range in ranges {
        let range_values = storage_guard.get_values(range.start, range.end);
        values.extend(range_values);
    }

    let response_array = values.iter().map(|val| val.as_str().to_string()).collect();

    Ok(Response::Array(response_array))
}
