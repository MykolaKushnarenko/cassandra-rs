//! Entry point for the server application.

mod server;
mod storage;
mod protocol_parser;
mod connection;
mod error;
mod handlers;

use server::Server;

/// Initializes and starts the server.
fn main() {
    let server = Server::new();
    server.start();
}
