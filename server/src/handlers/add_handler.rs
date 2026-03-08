//! Handler for the "add" command.

use crate::handlers::Handler;
use crate::storage::{GlobalStorage, Storage};
use shared::error::AppResult;
use shared::protocol::types::{Request, Response};

/// A handler that adds a value to the global storage.
pub(crate) struct AddHandler {
    storage: GlobalStorage,
}

impl Handler for AddHandler {
    fn handle(&mut self, request: Request) -> AppResult<Response> {
        let mut storage = self.storage.lock().unwrap();

        if matches!(request, Request::Add(_)) {
            if let Request::Add(value) = request {
                storage.add(value.clone());

                return Ok(Response::String(format!(
                    "Added {}, there are currently {}",
                    value,
                    storage.get_count()
                )));
            }
            return Ok(Response::String("Wrong handler!".to_string()));
        }

        Ok(Response::String("Wrong handler!".to_string()))
    }
}

impl AddHandler {
    /// Creates a new `AddHandler` with the given global storage.
    pub fn new(storage: GlobalStorage) -> Self {
        Self { storage }
    }
}
