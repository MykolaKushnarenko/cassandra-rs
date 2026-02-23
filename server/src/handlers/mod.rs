//! Command handlers for the server.
//!
//! This module defines the `Handler` trait and provides implementations for different commands.

use crate::error::AppResult;
use crate::server::OutboundMessage;

pub(crate) mod add_handler;
pub(crate) mod check_handler;

/// A trait for handling specific commands.
pub trait Handler<T> {
    /// Processes a command with the given value and returns an `OutboundMessage`.
    fn handle(&mut self, value: T) -> AppResult<OutboundMessage>;
}