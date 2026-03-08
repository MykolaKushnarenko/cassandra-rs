use crate::handlers::add_batch_handler::AddBatchHandler;
use crate::handlers::add_handler::AddHandler;
use crate::handlers::check_handler::CheckHandler;
use crate::handlers::drop_batch_handler::DropBatch;
use crate::handlers::get_batch_handler::GetBatchHandler;
use crate::handlers::get_count::GetCountHandler;
use crate::storage::GlobalStorage;
use shared::error::AppResult;
use shared::protocol::types::{Request, Response};

pub(crate) struct HandlerManager {
    add_batch_handler: AddBatchHandler,
    drop_batch_handler: DropBatch,
    get_batch_handler: GetBatchHandler,
    get_count_handler: GetCountHandler,
    add_handler: AddHandler,
    check_handler: CheckHandler,
}

impl HandlerManager {
    pub fn new(storage: GlobalStorage) -> Self {
        Self {
            add_batch_handler: AddBatchHandler::new(storage.clone()),
            drop_batch_handler: DropBatch::new(storage.clone()),
            get_batch_handler: GetBatchHandler::new(storage.clone()),
            get_count_handler: GetCountHandler::new(storage.clone()),
            add_handler: AddHandler::new(storage.clone()),
            check_handler: CheckHandler::new(storage.clone()),
        }
    }

    pub fn handle(&mut self, request: &Request) -> AppResult<Response> {
        match request {
            Request::Add(_) => self.add_handler.handle(&request),
            Request::Check(_) => self.check_handler.handle(&request),
            Request::GetBatch(_) => self.get_batch_handler.handle(&request),
            Request::DropBatch(_) => self.drop_batch_handler.handle(&request),
            Request::AddBatch(_) => self.add_batch_handler.handle(&request),
            Request::Count => self.get_count_handler.handle(&request),
        }
    }
}
