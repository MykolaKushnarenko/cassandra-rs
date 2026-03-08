//! Handler for the "get bach" command.

use crate::storage::{GlobalStorage, Storage};
use shared::error::AppResult;
use shared::protocol::types::{Request, Response};

/// A handler that gets a batch of values by the provided range of keys.
pub(crate) struct GetCountHandler {
    storage: GlobalStorage,
}

impl GetCountHandler {
    pub(crate) fn handle(&mut self, request: &Request) -> AppResult<Response> {
        let storage = self.storage.lock().unwrap();

        if matches!(request, Request::Count) {
            if let Request::Count = request {
                let count = storage.get_count();
                return Ok(Response::String(format!("Count: {}", count)));
            }
            return Ok(Response::String("Wrong handler!".to_string()));
        }

        Ok(Response::String("Wrong handler!".to_string()))
    }
}

impl GetCountHandler {
    /// Creates a new `GetCountHandler` with the given global storage.
    pub fn new(storage: GlobalStorage) -> Self {
        Self { storage }
    }
}
