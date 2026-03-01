//! Handler for the "drop batch" command.

use crate::handlers::Handler;
use crate::storage::{GlobalStorage, Storage};
use shared::error::AppResult;
use shared::protocol::types::{Request, Response};

/// A handler that drops value by the provided range of keys.
pub(crate) struct DropBatch<T> {
    storage: GlobalStorage<T>,
}

impl Handler for DropBatch<String> {
    fn handle(&mut self, value: Request) -> AppResult<Response> {
        let mut storage = self.storage.lock().unwrap();

        if matches!(value, Request::DropBatch(_)) {
            if let Request::DropBatch(ranges) = value {
                let mut count = 0;
                let mut keys = Vec::new();
                for range in ranges {
                    let range_values = storage.get_keys_in_range(range.start, range.end);
                    keys.extend(range_values);
                }

                for key in keys {
                    let result = storage.remove(key);
                    if result {
                        count += 1;
                    }
                }

                return Ok(Response::String(format!("Removed: {}", count,)));
            }

            return Ok(Response::String("Removed: 0".to_string()));
        }

        Ok(Response::String("Wrong handler!".to_string()))
    }
}

impl<T> DropBatch<T> {
    /// Creates a new `DropBatch` with the given global storage.
    pub fn new(storage: GlobalStorage<T>) -> Self {
        Self { storage }
    }
}
