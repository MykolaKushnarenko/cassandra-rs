//! Handler for the "check" command.

use std::hash::Hash;
use crate::error::AppResult;
use crate::handlers::Handler;
use crate::server::OutboundMessage;
use crate::storage::{GlobalStorage, Storage};

/// A handler that checks if a value exists in the global storage.
pub struct CheckHandler<T> {
    storage: GlobalStorage<T>
}

impl<T: Eq + Hash + std::fmt::Display> Handler<T> for CheckHandler<T> {
    fn handle(&mut self, value: T) -> AppResult<OutboundMessage> {
        let storage = self.storage.lock().unwrap();
        
        let result = storage.check(&value);
        if result {
            return  Ok(OutboundMessage { 
                status: "OK".to_string(),
                result: format!("Value: {} exists", value) })
        }

        Ok(OutboundMessage { 
            status: "ERROR".to_string(),
            result: format!("Value: {} doesn't exist", value) })
    }
}

impl<T> CheckHandler<T> {
    /// Creates a new `CheckHandler` with the given global storage.
    pub fn new(storage: GlobalStorage<T>) -> Self {
        Self { storage }
    }
}