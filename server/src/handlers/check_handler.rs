//! Handler for the "check" command.

use crate::storage::{GlobalStorage, Storage};
use shared::error::AppResult;
use shared::protocol::types::Response;

/// Checks if a value exists in the global storage.
pub(crate) fn handle(value: &String, storage: &GlobalStorage) -> AppResult<Response> {
    let storage_guard = storage.lock().unwrap();

    let exists = storage_guard.check(value);
    if exists {
        return Ok(Response::Bool(true));
    }

    Ok(Response::String(format!("Value: {} doesn't exist", value)))
}
