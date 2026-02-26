//! Handler for the "check" command.

use shared::error::AppResult;
use shared::protocol::types::Response;
use crate::handlers::Handler;
use crate::storage::{GlobalStorage, Storage};

/// A handler that checks if a value exists in the global storage.
pub struct CheckHandler<T> {
    storage: GlobalStorage<T>
}

impl Handler<String> for CheckHandler<String> {
    fn handle(&mut self, value: String) -> AppResult<Response> {
        let storage = self.storage.lock().unwrap();
        
        let result = storage.check(&value);
        if result {
            return Ok(Response::Bool(true));
        }

        Ok(Response::String(format!("Value: {} doesn't exist", value)))
    }
}

impl<T> CheckHandler<T> {
    /// Creates a new `CheckHandler` with the given global storage.
    pub fn new(storage: GlobalStorage<T>) -> Self {
        Self { storage }
    }
}