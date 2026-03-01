//! Command handlers for the server.
//!
//! This module defines the `Handler` trait and provides implementations for different commands.

use shared::error::AppResult;
use shared::protocol::types::{Request, Response};

pub(crate) mod add_handler;
pub(crate) mod add_batch_handler;
pub(crate) mod check_handler;
pub(crate) mod drop_batch_handler;
pub(crate) mod get_batch_handler;

/// A trait for handling specific commands.
pub trait Handler {
    /// Processes a command with the given value and returns an `OutboundMessage`.
    fn handle(&mut self, request: Request) -> AppResult<Response>;
}
