//! Entry point for the server application.

mod handler_manager;
mod handlers;
mod replicator;
mod server;
mod storage;

use server::Server;

/// Initializes and starts the server.
fn main() {
    env_logger::Builder::from_default_env()
        .filter_level(log::LevelFilter::Debug)
        .init();

    let mut server = Server::new();
    server.start();
}
