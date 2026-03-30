//! Handler for the "drop batch" command.

use crate::storage::{GlobalStorage, Storage};
use shared::consistent_hash_ring::Range;
use shared::error::AppResult;
use shared::protocol::types::Response;

/// Drops values by the provided range of keys.
pub(crate) fn handle(ranges: &[Range], storage: &GlobalStorage) -> AppResult<Response> {
    let mut storage_guard = storage.lock().unwrap();

    let mut count = 0;
    let mut keys = Vec::new();
    for range in ranges {
        let range_values = storage_guard.get_keys_in_range(range.start, range.end);
        keys.extend(range_values);
    }

    for key in keys {
        let result = storage_guard.remove(key);
        if result {
            count += 1;
        }
    }

    Ok(Response::String(format!("Removed: {}", count)))
}
