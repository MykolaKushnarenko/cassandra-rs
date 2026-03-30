//! Handler for the "add" command.

use crate::replicator::ReplicationEntry;
use crate::storage::{GlobalStorage, Storage};
use shared::error::AppResult;
use shared::protocol::types::{Entry, Response};
use std::sync::mpsc::Sender;

/// Adds a value to the global storage.
pub(crate) fn handle(
    entry: &Entry,
    storage: &GlobalStorage,
    sender: &Sender<ReplicationEntry>,
) -> AppResult<Response> {
    let mut storage_guard = storage.lock().unwrap();

    storage_guard.add(entry.value.clone());

    sender
        .send(ReplicationEntry::Single(entry.clone()))
        .unwrap();

    Ok(Response::String(format!(
        "Added {}, there are currently {}",
        entry.value,
        storage_guard.get_count()
    )))
}
