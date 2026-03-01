//! Handler for the "check" command.

use crate::handlers::Handler;
use crate::storage::{GlobalStorage, Storage};
use shared::error::AppResult;
use shared::protocol::types::{Request, Response};

/// A handler that checks if a value exists in the global storage.
pub struct CheckHandler<T> {
    storage: GlobalStorage<T>,
}

impl Handler for CheckHandler<String> {
    fn handle(&mut self, request: Request) -> AppResult<Response> {
        let storage = self.storage.lock().unwrap();

        if matches!(request, Request::Check(_)) {
            if let Request::Check(value_ti_check) = request {
                let result = storage.check(&value_ti_check);
                if result {
                    return Ok(Response::Bool(true));
                }

                return Ok(Response::String(format!(
                    "Value: {} doesn't exist",
                    value_ti_check
                )));
            }
        }

        Ok(Response::String("Wrong handler!".to_string()))
    }
}

impl<T> CheckHandler<T> {
    /// Creates a new `CheckHandler` with the given global storage.
    pub fn new(storage: GlobalStorage<T>) -> Self {
        Self { storage }
    }
}
