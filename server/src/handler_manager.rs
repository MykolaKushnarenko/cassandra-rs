use crate::handlers::{
    add_batch_handler, add_handler, check_handler, drop_batch_handler, get_batch_handler, get_count,
};
use crate::replicator::ReplicationEntry;
use crate::storage::GlobalStorage;
use shared::error::AppResult;
use shared::protocol::types::{Request, Response};
use std::sync::mpsc::Sender;

pub(crate) struct HandlerManager {
    storage: GlobalStorage,
    sender: Sender<ReplicationEntry>,
}

impl HandlerManager {
    pub(crate) fn new(storage: GlobalStorage, sender: Sender<ReplicationEntry>) -> Self {
        Self { storage, sender }
    }

    pub(crate) fn handle(&self, request: &Request) -> AppResult<Response> {
        match request {
            Request::Check(value) => check_handler::handle(value, &self.storage),
            Request::AddBatch(items) => {
                add_batch_handler::handle(items, &self.storage, &self.sender)
            }
            Request::Add(entry) => add_handler::handle(entry, &self.storage, &self.sender),
            Request::Count => get_count::handle(&self.storage),
            Request::DropBatch(ranges) => drop_batch_handler::handle(ranges, &self.storage),
            Request::GetBatch(ranges) => get_batch_handler::handle(ranges, &self.storage),
        }
    }
}
