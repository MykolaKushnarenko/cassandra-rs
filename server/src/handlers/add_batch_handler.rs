//! Handler for the "add batch" command.

use crate::storage::{GlobalStorage, Storage};
use shared::error::AppResult;
use shared::protocol::types::{Request, Response};

/// A handler that adds a batch of values to the global storage.
pub(crate) struct AddBatchHandler {
    storage: GlobalStorage,
}

impl AddBatchHandler {
    pub(crate) fn handle(&mut self, request: &Request) -> AppResult<Response> {
        let mut storage = self.storage.lock().unwrap();

        if matches!(request, Request::AddBatch(_)) {
            if let Request::AddBatch(items) = request {
                for item in items.iter() {
                    storage.add(item.clone());
                }

                return Ok(Response::String(format!(
                    "Inserted batch of {}",
                    items.len()
                )));
            }
            return Ok(Response::String("Wrong handler!".to_string()));
        }

        Ok(Response::String("Wrong handler!".to_string()))
    }
}

impl AddBatchHandler {
    /// Creates a new `AddBatchHandler` with the given global storage.
    pub fn new(storage: GlobalStorage) -> Self {
        Self { storage }
    }
}
