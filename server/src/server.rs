//! Core server logic and message handling.
//!
//! This module contains the `Server` struct which manages incoming TCP connections,
//! dispatches messages to appropriate handlers, and maintains global storage.

use crate::connection::Connection;
use crate::error::{AppResult, Error};
use crate::handlers::Handler;
use crate::storage::HashSetStorage;
use crate::{handlers, storage};
use add_handler::AddHandler;
use check_handler::CheckHandler;
use handlers::{add_handler, check_handler};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::net::{TcpListener, TcpStream};
use std::sync::Mutex;
use std::thread;
use storage::GlobalStorage;

/// The address of the server.
const LOCALHOST: &str = "localhost";

/// Represents a message received from a client.
#[derive(Debug, Deserialize)]
pub(crate) struct InboundMessage {
    /// The command name (e.g., "add", "check").
    pub command: String,
    /// Arguments for the command.
    pub args: Vec<String>,
}

/// Represents a message to be sent to a client.
#[derive(Debug, Serialize)]
pub(crate) struct OutboundMessage {
    /// The status of the operation (e.g., "OK", "ERROR").
    pub status: String,
    /// The result string to be returned to the client.
    pub result: String,
}

/// The main server struct.
///
/// It holds the shared storage and provides methods to start the server and process connections.
pub(crate) struct Server {
    storage: GlobalStorage<String>
}

impl Server {
    /// Creates a new `Server` instance with empty storage.
    pub(crate) fn new() -> Self {
        let storage = GlobalStorage::new(Mutex::new(HashSetStorage::<String>::new()));

        Self {
            storage
        }
    }

    /// Starts the server.
    ///
    /// It parses the port from command line arguments and begins listening for incoming connections.
    pub fn start(&self) {
        let port = Self::get_port(std::env::args().collect());

        let listener_result = TcpListener::bind(format!("{}:{}", LOCALHOST, port));

        println!("Server started on port {}", port);
        match listener_result {
            Ok(listener) => self.start_accepting(listener),
            Err(e) => {
                eprintln!("Error: {}", e)
            }
        }
    }

    /// Continuously accepts incoming TCP connections and spawns a new thread for each.
    fn start_accepting(&self, listener: TcpListener) {
        loop {
            let incoming_result = listener.accept();

            match incoming_result {
                Ok(_) => { },
                Err(e) => {
                    eprintln!("Error: {}", e);
                    continue;
                }
            }

            let (stream, client_address) = incoming_result.unwrap();

            let storage = self.storage.clone();

            thread::spawn(move || {
                println!("Accepted connection from: {}", client_address);

                let result = Self::handle_connection(stream, storage);

                match result {
                    Ok(_) => {},
                    Err(e) => {
                        eprintln!("Error occurred while processing message: {:?}", e);}
                }

                println!("Connection aborted for: {}", client_address);
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
    fn handle_connection(stream: TcpStream, storage: GlobalStorage<String>) -> AppResult<()> {
        let mut connection = Connection::new(&stream);
        let add_handler = AddHandler::new(storage.clone());
        let check_handler = CheckHandler::new(storage.clone());

        let mut handlers: HashMap<String, Box<dyn Handler<String>>> = HashMap::new();
        handlers.insert("add".to_string(), Box::new(add_handler));
        handlers.insert("check".to_string(), Box::new(check_handler));

        loop {
            Self::process_message(&mut connection, &mut handlers)?;
        }
    }

    /// Receives, processes, and responds to a single message from a connection.
    fn process_message(connection: &mut Connection, handlers:&mut HashMap<String, Box<dyn Handler<String>>>) -> AppResult<()> {
        let message = connection.receive::<InboundMessage>()?;

        let command = message.command.as_str();

        let look_up_handler = handlers.get_mut(command);
        let handler;
        
        match look_up_handler {
            Some(instance) => {handler = instance},
            None => {
                let error_response = OutboundMessage {
                    status: "ERROR".to_string(),
                    result: format!("Unknown command: {}", command)
                };
                connection.send(&error_response)?;
                return Err(Error::UnknownCommand(command.to_string()))
            }
        }

        if message.args.is_empty() || message.args.iter().filter(|arg| arg.parse::<f64>().is_err()).count() > 0 {
            let error_response = OutboundMessage {
                status: "ERROR".to_string(),
                result: "Value should be of number type".to_string()
            };
            connection.send(&error_response)?;
            return Err(Error::InvalidRequestContent);
        }

        let result =handler.handle(message.args.first().unwrap().to_string())?;
        connection.send(&result)?;
        
        Ok(())
    }
}