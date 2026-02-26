//! Handler for the "add" command.

use std::fmt::Display;
use std::hash::Hash;
use crate::error::AppResult;
use crate::handlers::Handler;
use crate::server::OutboundMessage;
use crate::storage::{GlobalStorage, Storage};

/// A handler that adds a value to the global storage.
pub(crate) struct AddHandler<T> {
    storage: GlobalStorage<T>,
}

impl<T: Eq + Hash + Display + Clone> Handler<T> for AddHandler<T> {
    fn handle(&mut self, value: T) -> AppResult<OutboundMessage> {
        let mut storage = self.storage.lock().unwrap();

        storage.add(value.clone());

        Ok(OutboundMessage {
            status: "OK".to_string(),
            result: format!("Added {}, there are currently {}", value, storage.get_count())
        })
    }
}

impl<T> AddHandler<T> {
    /// Creates a new `AddHandler` with the given global storage.
    pub fn new(storage: GlobalStorage<T>) -> Self {
        Self { storage }
    }
}