//! Handler for the "add" command.

use crate::handlers::Handler;
use crate::storage::{GlobalStorage, Storage};
use shared::error::AppResult;
use shared::protocol::types::Response;

/// A handler that adds a value to the global storage.
pub(crate) struct AddHandler<T> {
    storage: GlobalStorage<T>,
}

impl Handler<String> for AddHandler<String> {
    fn handle(&mut self, value: String) -> AppResult<Response> {
        let mut storage = self.storage.lock().unwrap();

        storage.add(value.clone());

        Ok(Response::String(format!("Added {}, there are currently {}", value, storage.get_count())))
    }
}

impl<T> AddHandler<T> {
    /// Creates a new `AddHandler` with the given global storage.
    pub fn new(storage: GlobalStorage<T>) -> Self {
        Self { storage }
    }
}