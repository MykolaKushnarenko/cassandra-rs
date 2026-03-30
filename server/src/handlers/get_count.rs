//! Handler for the "get count" command.

use crate::storage::{GlobalStorage, Storage};
use shared::error::AppResult;
use shared::protocol::types::Response;

/// Gets the count of values in storage.
pub(crate) fn handle(storage: &GlobalStorage) -> AppResult<Response> {
    let storage_guard = storage.lock().unwrap();
    let count = storage_guard.get_count();
    Ok(Response::String(format!("Count: {}", count)))
}
