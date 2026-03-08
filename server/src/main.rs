//! Entry point for the server application.

mod handlers;
mod server;
mod storage;

use server::Server;

/// Initializes and starts the server.
fn main() {
    let server = Server::new();
    server.start();
}
