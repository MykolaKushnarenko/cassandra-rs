//! Handler for the "add batch" command.

use crate::replicator::ReplicationEntry;
use crate::storage::{GlobalStorage, Storage};
use shared::error::AppResult;
use shared::protocol::types::{Entry, Response};
use std::sync::mpsc::Sender;

/// Adds a batch of values to the global storage.
pub(crate) fn handle(
    items: &[Entry],
    storage: &GlobalStorage,
    sender: &Sender<ReplicationEntry>,
) -> AppResult<Response> {
    let mut storage_guard = storage.lock().unwrap();

    let items_count = items.len();
    for item in items.iter() {
        storage_guard.add(item.value.clone());
    }

    sender
        .send(ReplicationEntry::Batch(items.to_vec()))
        .unwrap();

    Ok(Response::String(format!(
        "Inserted batch of {}",
        items_count
    )))
}
