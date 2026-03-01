//! Handler for the "get bach" command.

use crate::handlers::Handler;
use crate::storage::{GlobalStorage, Storage};
use shared::error::AppResult;
use shared::protocol::types::{Request, Response};

/// A handler that gets a batch of values by the provided range of keys.
pub(crate) struct GetBatchHandler<T> {
    storage: GlobalStorage<T>,
}

impl Handler for GetBatchHandler<String> {
    fn handle(&mut self, request: Request) -> AppResult<Response> {
        let storage = self.storage.lock().unwrap();

        if matches!(request, Request::GetBatch(_)) {
            if let Request::GetBatch(ranges) = request {

                let mut values = Vec::new();
                for range in ranges {
                    let range_values = storage.get_values(range.start, range.end);
                    values.extend(range_values);
                }

                let response_array = values.iter().map(|val| val.as_str().to_string()).collect();

                return Ok(Response::Array(response_array));
            }
            return Ok(Response::String("Wrong handler!".to_string()));
        }

        Ok(Response::String("Wrong handler!".to_string()))
    }
}

impl<T> GetBatchHandler<T> {
    /// Creates a new `GetBatchHandler` with the given global storage.
    pub fn new(storage: GlobalStorage<T>) -> Self {
        Self { storage }
    }
}
