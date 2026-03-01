//! Handler for the "add batch" command.

use crate::handlers::Handler;
use crate::storage::{GlobalStorage, Storage};
use shared::error::AppResult;
use shared::protocol::types::{Request, Response};

/// A handler that adds a batch of values to the global storage.
pub(crate) struct AddBatchHandler<T> {
    storage: GlobalStorage<T>,
}

impl Handler for AddBatchHandler<String> {
    fn handle(&mut self, request: Request) -> AppResult<Response> {
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

impl<T> AddBatchHandler<T> {
    /// Creates a new `AddBatchHandler` with the given global storage.
    pub fn new(storage: GlobalStorage<T>) -> Self {
        Self { storage }
    }
}
