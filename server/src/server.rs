//! Core server logic and message handling.
//!
//! This module contains the `Server` struct which manages incoming TCP connections,
//! dispatches messages to appropriate handlers, and maintains global storage.

use crate::handler_manager::HandlerManager;
use crate::replicator::{ReplicationEntry, Replicator};
use crate::storage;
use crate::storage::{BTreeStorage, Storage};
use log::{info, warn};
use shared::cluster::{Cluster, Node};
use shared::connection::Connection;
use shared::connection_pool::ConnectionPool;
use shared::error::AppResult;
use std::net::{TcpListener, TcpStream};
use std::sync::mpsc::{Receiver, Sender};
use std::sync::{Mutex, mpsc};
use std::thread;
use storage::GlobalStorage;

/// The address of the server.
const LOCALHOST: &str = "localhost";

const NODES_ARG_KEY: &str = "nodes=";

/// The main server struct.
///
/// It holds the shared storage and provides methods to start the server and process connections.
pub(crate) struct Server {
    storage: GlobalStorage,
}

impl Server {
    /// Creates a new `Server` instance with empty storage.
    pub(crate) fn new() -> Self {
        let storage = GlobalStorage::new(Mutex::new(BTreeStorage::new()));

        Self { storage }
    }

    /// Starts the server.
    ///
    /// It parses the port from command line arguments and begins listening for incoming connections.
    pub fn start(&mut self) {
        let port = Self::get_port(std::env::args().collect());

        let nodes = Self::initialize_cluster_config();
        let cluster = Cluster::new(nodes);

        let (tx, rx): (Sender<ReplicationEntry>, Receiver<ReplicationEntry>) = mpsc::channel();

        let mut replicator = Replicator::new(
            format!("localhost:{}", port).to_string(),
            cluster,
            rx,
            ConnectionPool::new(),
        );
        thread::spawn(move || replicator.run());

        let listener_result = TcpListener::bind(format!("{}:{}", LOCALHOST, port));

        info!("Server started on port {}", port);
        match listener_result {
            Ok(listener) => self.start_accepting(listener, tx),
            Err(e) => {
                warn!("Error: {}", e)
            }
        }
    }

    /// Continuously accepts incoming TCP connections and spawns a new thread for each.
    fn start_accepting(&self, listener: TcpListener, sender: Sender<ReplicationEntry>) {
        loop {
            let incoming_result = listener.accept();

            match incoming_result {
                Ok(_) => {}
                Err(e) => {
                    warn!("Error: {}", e);
                    continue;
                }
            }

            let (stream, client_address) = incoming_result.unwrap();

            let storage = self.storage.clone();

            let sender = sender.clone();
            thread::spawn(move || {
                info!("Accepted connection from: {}", client_address);

                let result = Self::handle_connection(stream, storage, sender);

                match result {
                    Ok(_) => {}
                    Err(e) => {
                        warn!("Error occurred while processing message: {:?}", e);
                    }
                }

                info!("Connection aborted for: {}", client_address);
            });
        }
    }

    /// Parses the port number from a command line argument string.
    fn parse_port(arg: &String) -> i32 {
        let port_string = arg.split("=").nth(1);
        port_string.unwrap().parse::<i32>().unwrap()
    }

    /// Retrieves the port number from command line arguments, defaulting to 4000 if not specified.
    fn get_port(args: Vec<String>) -> i32 {
        args.iter()
            .find(|arg| arg.starts_with("port"))
            .map(Self::parse_port)
            .unwrap_or(4000)
    }

    /// Handles an individual TCP connection.
    ///
    /// It initializes handlers and enters a loop to process messages from the client.
    fn handle_connection(
        stream: TcpStream,
        storage: GlobalStorage,
        sender: Sender<ReplicationEntry>,
    ) -> AppResult<()> {
        let mut connection = Connection::new(stream);
        let handler_manager = HandlerManager::new(storage, sender);

        loop {
            Self::process_message(&mut connection, &handler_manager)?;
        }
    }

    /// Receives, processes, and responds to a single message from a connection.
    fn process_message(
        connection: &mut Connection,
        handler_manager: &HandlerManager,
    ) -> AppResult<()> {
        let request = connection.receive_request()?;

        let handler_result = handler_manager.handle(&request)?;

        connection.send_response(&handler_result)?;

        Ok(())
    }

    fn initialize_cluster_config() -> Vec<Node> {
        let nodes = std::env::args()
            .inspect(|arg| info!("Arg: {}", arg))
            .find(|arg| arg.starts_with(NODES_ARG_KEY))
            .expect("--nodes argument is required")
            .trim_start_matches(NODES_ARG_KEY)
            .split(",")
            .map(|s| Node {
                address: s.to_string(),
            })
            .collect::<Vec<Node>>();

        info!("Nodes: {:?}", nodes);

        nodes
    }
}
